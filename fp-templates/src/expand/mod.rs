use crate::types::{TemplateParameter, TemplateParameterType};
use crate::FIBERPLANE_LIBRARY_PATH;
use fiberplane::protocols::core::NewNotebook;
use jrsonnet_evaluator::error::LocError;
use jrsonnet_evaluator::trace::{CompactFormat, ExplainingFormat, PathResolver, TraceFormat};
use jrsonnet_evaluator::{EvaluationState, FuncVal, IStr, ImportResolver, ManifestFormat, Val};
use jrsonnet_types::ValType;
use serde_json::{Number, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{any::Any, convert::AsRef, iter::IntoIterator, rc::Rc};

#[cfg(test)]
mod tests;

static FIBERPLANE_LIBRARY: &str = include_str!("../../fiberplane.libsonnet");
static PROXY_DATA_SOURCES_EXT_VAR: &str = "PROXY_DATA_SOURCES";

/// This can be passed to `expand_template` as the `args` parameter.
// Note: we provide this because the expansion functions take generic parameters
// so simply passing HashMap::new() results in a compiler error saying you
// need to specify the type annotations.
pub const EMPTY_ARGS: [(&str, Value); 0] = [];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing required argument: {0}")]
    MissingArgument(String),

    #[error("{0}")]
    Evaluation(String),

    #[error("template did not produce a valid notebook: {0:?}")]
    InvalidOutput(#[from] serde_json::Error),
}

/// Evaluate the template with the given top-level arguments.
///
/// This function also filters out any invalid labels to ensure
/// that the notebook will be successfully created.
pub fn expand_template(
    template: impl AsRef<str>,
    args: impl IntoIterator<Item = (impl AsRef<str>, impl Into<Value>)>,
) -> Result<NewNotebook, Error> {
    TemplateExpander::default().expand_template(template, args)
}

/// Extract the template parameters (if the template exports a top-level function)
pub fn extract_template_parameters(
    template: impl AsRef<str>,
) -> Result<Vec<TemplateParameter>, Error> {
    TemplateExpander::default().extract_template_parameters(template)
}

#[derive(Default)]
pub struct TemplateExpander {
    max_stack: Option<usize>,
    proxy_data_sources: Option<Value>,
    explaining_traces: bool,
}

impl TemplateExpander {
    pub fn new(max_stack: Option<usize>, proxy_data_sources: Option<Value>) -> Self {
        Self {
            max_stack,
            proxy_data_sources,
            ..Default::default()
        }
    }

    /// Display detailed rustc-style error messages that explain
    /// the problem (defaults to false)
    pub fn set_explaining_traces(&mut self, explaining_traces: bool) {
        self.explaining_traces = explaining_traces;
    }

    /// Set the data sources that are available to the template.
    pub fn set_proxy_data_sources(&mut self, proxy_data_sources: Value) {
        self.proxy_data_sources = Some(proxy_data_sources);
    }

    /// Evaluate the template with the given top-level arguments.
    ///
    /// This function also filters out any invalid labels to ensure
    /// that the notebook will be successfully created.
    pub fn expand_template(
        &self,
        template: impl AsRef<str>,
        args: impl IntoIterator<Item = (impl AsRef<str>, impl Into<Value>)>,
    ) -> Result<NewNotebook, Error> {
        let string = self.expand_template_to_string(template, args, false)?;
        let mut notebook: NewNotebook = serde_json::from_str(&string)?;

        // Filter out any invalid labels
        notebook.labels.retain(|label| label.validate().is_ok());

        Ok(notebook)
    }

    /// Evaluate the template with the given top-level arguments.
    ///
    /// Note this method is private so that we can ensure that the
    /// notebook created by the template is valid before returning it.
    pub(crate) fn expand_template_to_string(
        &self,
        template: impl AsRef<str>,
        args: impl IntoIterator<Item = (impl AsRef<str>, impl Into<Value>)>,
        pretty_print: bool,
    ) -> Result<String, Error> {
        let (state, result) = self.expand_template_inner(template)?;

        let num_spaces = if pretty_print { 2 } else { 0 };
        state.set_manifest_format(ManifestFormat::Json(num_spaces));

        // If the top-level return is a function, add the top level arguments
        // that correspond to parameters of that function.
        // (If the function does not accept a parameter that we were given,
        // we will not pass it in rather than letting the function fail)
        let result = if let Val::Func(func) = &result {
            if let FuncVal::Normal(func) = func.as_ref() {
                let params: HashSet<&str> = func.params.0.iter().map(|p| &*p.0).collect();

                for (name, value) in args.into_iter() {
                    let name = name.as_ref();
                    let value = value.into();
                    if params.contains(name) {
                        if let Value::String(value) = value {
                            state.add_tla_str(name.into(), value.as_str().into());
                        } else {
                            // Inject the value as code so the jsonnet library parses it as JSON
                            state
                                .add_tla_code(
                                    name.into(),
                                    serde_json::to_string(&value)?.as_str().into(),
                                )
                                .map_err(|err| self.format_trace(&state, err))?;
                        }
                    }
                }
            }

            // Run the function with the TLAs we just added
            state.with_tla(result).map_err(|err| {
                if let jrsonnet_evaluator::error::Error::FunctionParameterNotBoundInCall(param) =
                    err.error()
                {
                    Error::MissingArgument(param.to_string())
                } else {
                    self.format_trace(&state, err)
                }
            })?
        } else {
            result
        };

        Ok(state
            .manifest(result)
            .map_err(|err| self.format_trace(&state, err))?
            .to_string())
    }

    /// Extract the template parameters (if the template exports a top-level function)
    pub fn extract_template_parameters(
        &self,
        template: impl AsRef<str>,
    ) -> Result<Vec<TemplateParameter>, Error> {
        let (state, result) = self.expand_template_inner(template)?;
        if let Val::Func(func) = result {
            if let FuncVal::Normal(func) = func.as_ref() {
                return Ok(func
                    .params
                    .iter()
                    .map(|param| {
                        if let Some(expr) = &param.1 {
                            // Evaluate the parameter expression to determine the final value
                            let parameter_result = state.run_in_state(|| {
                                jrsonnet_evaluator::evaluate(func.ctx.clone(), expr)
                            });
                            if let Ok(val) = parameter_result {
                                let ty = match val.value_type() {
                                    ValType::Str => TemplateParameterType::String,
                                    ValType::Num => TemplateParameterType::Number,
                                    ValType::Bool => TemplateParameterType::Boolean,
                                    ValType::Arr => TemplateParameterType::Array,
                                    ValType::Obj => TemplateParameterType::Object,
                                    ValType::Null => TemplateParameterType::Unknown,
                                    ValType::Func => TemplateParameterType::Unknown,
                                };
                                return TemplateParameter {
                                    name: param.0.to_string(),
                                    default_value: jsonnet_val_to_json_value(val),
                                    ty,
                                };
                            }
                        }
                        TemplateParameter {
                            name: param.0.to_string(),
                            default_value: None,
                            ty: TemplateParameterType::Unknown,
                        }
                    })
                    .collect());
            }
        }
        Ok(Vec::new())
    }

    /// Inject the external variables and evaluate the template.
    /// If the template exports a function, the returned Val will be a FuncVal
    /// and it will need to be evaluated again against the Top-Level Arguments.
    fn expand_template_inner(
        &self,
        template: impl AsRef<str>,
    ) -> Result<(EvaluationState, Val), Error> {
        let state = EvaluationState::default();
        state.with_stdlib();
        state.set_import_resolver(Box::new(YouCanCheckOutAnyLibYouWantButOnlyFiberNet));
        if let Some(stack_size) = self.max_stack {
            state.set_max_stack(stack_size);
        }

        // Inject the data sources as JSON
        let data_sources = if let Some(data_sources) = &self.proxy_data_sources {
            serde_json::to_string(data_sources)?
        } else {
            "[]".to_string()
        };
        state
            .add_ext_code(
                PROXY_DATA_SOURCES_EXT_VAR.into(),
                data_sources.as_str().into(),
            )
            .map_err(|err| self.format_trace(&state, err))?;

        let result = state
            .evaluate_snippet_raw(PathBuf::from("template").into(), template.as_ref().into())
            .map_err(|err| self.format_trace(&state, err))?;

        Ok((state, result))
    }

    // Stringify LocErrors so that they include the correct line numbers
    fn format_trace(&self, state: &EvaluationState, err: LocError) -> Error {
        let mut message = String::new();
        let result = if self.explaining_traces {
            ExplainingFormat {
                resolver: PathResolver::FileName,
            }
            .write_trace(&mut message, state, &err)
        } else {
            CompactFormat {
                resolver: PathResolver::FileName,
                padding: 2,
            }
            .write_trace(&mut message, state, &err)
        };

        let message = match result {
            Ok(_) => message,
            Err(err) => err.to_string(),
        };
        Error::Evaluation(message)
    }
}

/// This is an import resolver that only loads the fiberplane library.
///
/// It works for imports of any path that ends with the filename "fiberplane.libsonnet".
struct YouCanCheckOutAnyLibYouWantButOnlyFiberNet;

impl ImportResolver for YouCanCheckOutAnyLibYouWantButOnlyFiberNet {
    fn resolve_file(&self, from: &Path, path: &Path) -> Result<Rc<Path>, LocError> {
        match path.file_name() {
            Some(filename) if filename == FIBERPLANE_LIBRARY_PATH => {
                Ok(PathBuf::from(FIBERPLANE_LIBRARY_PATH).into())
            }
            _ => Err(LocError::new(
                jrsonnet_evaluator::error::Error::ImportNotSupported(
                    from.to_owned(),
                    path.to_owned(),
                ),
            )),
        }
    }

    fn load_file_contents(&self, resolved: &Path) -> Result<IStr, LocError> {
        match resolved.file_name() {
            Some(filename) if filename == FIBERPLANE_LIBRARY_PATH => Ok(FIBERPLANE_LIBRARY.into()),
            _ => Err(LocError::new(
                jrsonnet_evaluator::error::Error::ImportNotSupported(
                    PathBuf::new(),
                    resolved.to_owned(),
                ),
            )),
        }
    }

    unsafe fn as_any(&self) -> &dyn Any {
        panic!("this resolver can't be used as any")
    }
}

/// Convert a Jsonnet "Val" into a serde_json::Value
fn jsonnet_val_to_json_value(val: Val) -> Option<Value> {
    match val {
        Val::Bool(b) => Some(Value::Bool(b)),
        Val::Str(s) => Some(Value::String(s.to_string())),
        Val::Num(n) => Number::from_f64(n).map(Value::Number),
        Val::Obj(o) => {
            let obj = o
                .fields()
                .into_iter()
                .flat_map(|key| {
                    let val = o.get(key.clone()).ok().flatten()?;
                    Some((key.to_string(), jsonnet_val_to_json_value(val)?))
                })
                .collect();
            Some(Value::Object(obj))
        }
        Val::Arr(a) => {
            let arr = a
                .iter()
                .flatten()
                .flat_map(jsonnet_val_to_json_value)
                .collect();
            Some(Value::Array(arr))
        }
        Val::Null => Some(Value::Null),
        Val::Func(_) => None,
    }
}
