use self::code_writer::CodeWriter;
use self::escape_string::escape_string;
use crate::FIBERPLANE_LIBRARY_PATH;
use fiberplane::protocols::{
    core::{Cell, DataSource, HeadingType, ListType, NewNotebook, NotebookDataSource},
    formatting::{Annotation, AnnotationWithOffset, Formatting},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;

mod code_writer;
mod escape_string;
#[cfg(test)]
mod tests;

lazy_static! {
    static ref MUSTACHE_SUBSTITUTION: Regex = Regex::new(r"\{\{(\w+)\}\}").unwrap();
}

// Note: we use the NewNotebook struct because it contains
// the subset of the Notebook fields that we need to
// create a template from it.
pub fn notebook_to_template(notebook: impl Into<NewNotebook>) -> String {
    let notebook = notebook.into();

    // We assume that the time range for the template should be
    // the same duration as the notebook's time range, but it
    // will be updated so the "to" value is the time when the
    // template is evaluated.
    let duration = notebook.time_range.to - notebook.time_range.from;
    let duration_minutes = (duration / 60.0).round();

    let mut writer = CodeWriter::new();

    // Write the preamble
    writer.println("// For documentation on Fiberplane Templates, see: https://github.com/fiberplane/templates");
    writer.println(format!("local fp = import '{}';", FIBERPLANE_LIBRARY_PATH));
    writer.println("local c = fp.cell;");
    writer.println("local fmt = fp.format;");
    writer.println("");
    writer.println("function(");
    writer.indent();

    // Add all of the variables from the title and content of the notebook
    let template_function_parameters = parse_template_function_parameters(
        &notebook.title,
        notebook.cells.iter().flat_map(|cell| cell.content()),
    );
    // If there aren't any parameters, add the title as an example of how to use template parameters
    let include_title_as_parameter = template_function_parameters.is_empty();

    for parameter_declaration in template_function_parameters {
        writer.println(parameter_declaration);
    }
    if include_title_as_parameter {
        writer.println(format!("title={}", escape_string(&notebook.title)));
    }

    // Close the template function signature
    writer.dedent();
    writer.println(")");
    writer.indent();
    writer.println("fp.notebook");

    // Add the title
    if include_title_as_parameter {
        // This references the `title` parameter added above
        writer.println(".new(title)");
    } else {
        writer.println(format!(
            ".new({})",
            escape_string_and_replace_mustache_substitutions(&notebook.title, " + ")
        ));
    }

    // Add the time range
    writer.println(format!(
        ".setTimeRangeRelative(minutes={})",
        duration_minutes
    ));

    // Add labels
    if notebook.labels.is_empty() {
        writer.println(r#".addLabels({})"#);
    } else {
        writer.println(".addLabels({");
        for label in notebook.labels {
            writer.println(format!(
                "{}: {},",
                escape_string(&label.key),
                escape_string(&label.value)
            ));
        }
        writer.println("})");
    }

    // Add data sources
    for (name, data_source) in notebook.data_sources {
        print_notebook_data_source(&mut writer, &name, &data_source);
    }

    // Add cells
    writer.println(".addCells([");
    writer.indent();
    for cell in notebook.cells {
        print_cell(&mut writer, cell);
    }
    writer.dedent();
    writer.println("])");
    writer.println("");

    writer.to_string()
}

/// Print the cell.
///
/// We try to print the cell in the most compact form that is still readable.
/// If it only has 1-2 properties, we print it on a single line.
/// If it has more, we print it on multiple lines and write out each property name.
fn print_cell(writer: &mut CodeWriter, cell: Cell) {
    let mut args = Vec::with_capacity(5);

    // Get the helper function name, arguments, and read only property from each cell
    // (read_only is handled separately because every cell has it)
    let (function_name, read_only) = match cell {
        Cell::Checkbox(cell) => {
            args.push(("content", format_content(&cell.content, cell.formatting)));
            args.push(("checked", cell.checked.to_string()));
            if let Some(level) = cell.level {
                args.push(("level", level.to_string()));
            }

            ("checkbox", cell.read_only)
        }
        Cell::Code(cell) => {
            args.push(("content", escape_string(&cell.content)));
            if let Some(syntax) = &cell.syntax {
                args.push(("syntax", escape_string(syntax)));
            }
            ("code", cell.read_only)
        }
        Cell::Divider(cell) => ("divider", cell.read_only),
        Cell::Heading(cell) => {
            let heading_type = match cell.heading_type {
                HeadingType::H1 => "h1",
                HeadingType::H2 => "h2",
                HeadingType::H3 => "h3",
            };
            args.push(("content", format_content(&cell.content, cell.formatting)));
            (heading_type, cell.read_only)
        }
        Cell::ListItem(cell) => {
            let function_name = match cell.list_type {
                ListType::Ordered => "listItem.ordered",
                ListType::Unordered => "listItem.unordered",
            };
            args.push(("content", format_content(&cell.content, cell.formatting)));
            if let Some(level) = cell.level {
                args.push(("level", level.to_string()));
            }
            if let Some(start_number) = cell.start_number {
                args.push(("startNumber", start_number.to_string()));
            }
            (function_name, cell.read_only)
        }
        Cell::Prometheus(cell) => {
            args.push(("content", escape_string(&cell.content)));
            ("prometheus", cell.read_only)
        }
        Cell::Elasticsearch(cell) => {
            args.push(("content", escape_string(&cell.content)));
            ("elasticsearch", cell.read_only)
        }
        Cell::Text(cell) => {
            args.push(("content", format_content(&cell.content, cell.formatting)));
            ("text", cell.read_only)
        }
        Cell::Image(cell) => {
            if let Some(url) = &cell.url {
                args.push(("url", escape_string(url)));
            }
            ("image", cell.read_only)
        }
        // Ignore other cell types
        _ => return,
    };

    // Only print the read only property if it's true
    if read_only == Some(true) {
        args.push(("readOnly", "true".to_string()));
    }

    // Print the cell on one line or multiple depending on how many properties it has
    let first_param = args.first().map(|(name, _)| *name);
    match (args.len(), first_param) {
        (0, _) => writer.println(format!("c.{}(),", function_name)),
        (1, Some("content")) => writer.println(format!("c.{}({}),", function_name, args[0].1)),
        (1, _) => writer.println(format!("c.{}({}={}),", function_name, args[0].0, args[0].1)),
        (2, Some("content")) => writer.println(format!(
            "c.{}({}, {}={}),",
            function_name, args[0].1, args[1].0, args[1].1
        )),
        _ => {
            writer.println(format!("c.{}(", function_name));
            writer.indent();
            for (param, val) in args {
                writer.println(format!("{}={},", param, val));
            }
            writer.dedent();
            writer.println("),")
        }
    };
}

fn format_content(content: &str, formatting: Option<Formatting>) -> String {
    match formatting {
        Some(mut formatting) if !formatting.is_empty() => {
            let mut output = "[".to_string();
            let mut index: usize = 0;
            // Count the number of starting and ending annotations to handle unmatched start annotations
            let mut start_annotations = 0;
            let mut end_annotations = 0;

            formatting.sort_by_key(|f| f.offset);

            // Convert each annotation to a jsonnet helper function
            for AnnotationWithOffset { offset, annotation } in formatting {
                let offset = offset as usize;
                // Add any content before this annotation to the output
                if offset > index {
                    output.push_str(&escape_string_and_replace_mustache_substitutions(
                        &content[index..offset],
                        ", ",
                    ));
                    output.push_str(", ");
                    index = offset;
                }
                match annotation {
                    Annotation::StartBold => {
                        output.push_str("fmt.bold([");
                        start_annotations += 1;
                    }
                    Annotation::StartCode => {
                        output.push_str("fmt.code([");
                        start_annotations += 1;
                    }
                    Annotation::StartItalics => {
                        output.push_str("fmt.italics([");
                        start_annotations += 1;
                    }
                    Annotation::StartStrikethrough => {
                        output.push_str("fmt.strikethrough([");
                        start_annotations += 1;
                    }
                    Annotation::StartUnderline => {
                        output.push_str("fmt.underline([");
                        start_annotations += 1;
                    }
                    Annotation::StartHighlight => {
                        output.push_str("fmt.highlight([");
                        start_annotations += 1;
                    }
                    Annotation::StartLink { url } => {
                        output.push_str("fmt.link(url=");
                        output.push_str(&escape_string(url));
                        output.push_str(", content=[");
                        start_annotations += 1;
                    }
                    Annotation::EndBold
                    | Annotation::EndCode
                    | Annotation::EndItalics
                    | Annotation::EndStrikethrough
                    | Annotation::EndUnderline
                    | Annotation::EndHighlight
                    | Annotation::EndLink => {
                        finish_enclosure(&mut output, "]), ");
                        end_annotations += 1;
                    }
                    Annotation::Mention(mention) => output.push_str(&format!("@{}", mention.name)),
                }
            }
            // If the content ends with plain text, make sure to add it to the output
            if index < content.len() {
                output.push_str(&escape_string_and_replace_mustache_substitutions(
                    &content[index..],
                    ", ",
                ));
            }
            // If there are unclosed annotations, add extra closing brackets and parens
            // to close the helper function calls
            if start_annotations > end_annotations {
                for _i in 0..start_annotations - end_annotations {
                    finish_enclosure(&mut output, "])")
                }
            }
            finish_enclosure(&mut output, "]");

            output
        }
        _ => escape_string(content),
    }
}

/// Remove trailing commas and whitespace and add the closing brackets/parens
fn finish_enclosure(string: &mut String, brackets: &str) {
    if string.ends_with(", ") {
        string.pop();
        string.pop();
    }
    string.push_str(brackets);
}

fn print_notebook_data_source(
    writer: &mut CodeWriter,
    name: &str,
    data_source: &NotebookDataSource,
) {
    match data_source {
        NotebookDataSource::Inline(inline) => print_data_source(writer, name, &inline.data_source),
        NotebookDataSource::Organization(org) => {
            print_data_source(writer, &org.name, &org.data_source)
        }
    }
}

fn print_data_source(writer: &mut CodeWriter, name: &str, data_source: &DataSource) {
    match data_source {
        DataSource::Proxy(proxy) => {
            writer
                .println(".addProxyDataSource(")
                .println_indented(format!("alias={},", escape_string(name)))
                .println_indented(format!("name={},", escape_string(&proxy.data_source_name)))
                .println_indented(format!("proxyId={},", escape_string(&proxy.proxy_id)))
                .println_indented(format!(
                    "type={}",
                    escape_string(proxy.data_source_type.to_string())
                ))
                .println(")");
        }
        DataSource::Prometheus(prometheus) => {
            writer
                .println(".addDirectDataSource(")
                .println_indented("type='prometheus',")
                .println_indented(format!("name={},", escape_string(name)))
                .println_indented("config={")
                .indent()
                .println_indented(format!("url: {},", escape_string(&prometheus.url)))
                .println("},")
                .dedent()
                .println(")");
        }
        DataSource::Elasticsearch(elasticsearch) => {
            writer
                .println(".addDirectDataSource(")
                .println_indented("type='elasticsearch',")
                .println_indented(format!("name={},", escape_string(name)))
                .println_indented("config={")
                .indent()
                .println_indented(format!("url: {},", escape_string(&elasticsearch.url)))
                .println_indented(format!(
                    "timestampFieldNames: [{}],",
                    elasticsearch
                        .timestamp_field_names
                        .iter()
                        .map(escape_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .println_indented(format!(
                    "bodyFieldNames: [{}],",
                    elasticsearch
                        .body_field_names
                        .iter()
                        .map(escape_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .println("},")
                .dedent()
                .println(")");
        }
        DataSource::Loki(loki) => {
            writer
                .println(".addDirectDataSource(")
                .println_indented("type='loki',")
                .println_indented(format!("name={},", escape_string(name)))
                .println_indented("config={")
                .indent()
                .println_indented(format!("url: {},", escape_string(&loki.url)))
                .println("},")
                .dedent()
                .println(")");
        }
        DataSource::Sentry(sentry) => {
            writer
                .println(".addDirectDataSource(")
                .println_indented("type='sentry',")
                .println_indented(format!("name={},", escape_string(name)))
                .println_indented("config={")
                .indent()
                .println_indented(format!("token: {},", escape_string(&sentry.token)))
                .println_indented(format!(
                    "organizationSlug: {},",
                    escape_string(&sentry.organization_slug)
                ))
                .println_indented(format!(
                    "projectSlug: {},",
                    escape_string(&sentry.project_slug)
                ))
                .println("},")
                .dedent()
                .println(")");
        }
    }
}

fn escape_string_and_replace_mustache_substitutions(content: &str, separator: &str) -> String {
    let escaped = escape_string(content);
    if let Some(quote) = escaped.chars().next() {
        let replaced = MUSTACHE_SUBSTITUTION.replace_all(
            &escaped,
            format!(
                "{quote}{separator}$1{separator}{quote}",
                quote = quote,
                separator = separator
            ),
        );
        // Remove empty strings (which can happen if the mustache substitution happens at the beginning or
        // end of the string, or if there are multiple substitutions in a row)
        replaced
            .replace(
                &format!(
                    "{quote}{quote}{separator}",
                    quote = quote,
                    separator = separator
                ),
                "",
            )
            .replace(
                &format!(
                    "{separator}{quote}{quote}",
                    quote = quote,
                    separator = separator
                ),
                "",
            )
    } else {
        escaped
    }
}

/// Parse the template function parameters from the title and cells and return
/// their jsonnet declarations in the form "parameter=defaultValue".
///
/// Note this will preserve the order in which the parameters are first used
fn parse_template_function_parameters<'a>(
    title: &str,
    cell_content: impl Iterator<Item = &'a str>,
) -> Vec<String> {
    let mut unique_parameters = BTreeSet::new();
    let title_substitutions = MUSTACHE_SUBSTITUTION.captures_iter(title);
    let cell_substitutions = cell_content.flat_map(|c| MUSTACHE_SUBSTITUTION.captures_iter(c));
    title_substitutions
        .chain(cell_substitutions)
        .flat_map(|c| c.get(1))
        .flat_map(|variable| {
            let variable = variable.as_str();
            if unique_parameters.insert(variable) {
                // This will print "variable={{variable}}" (the extra braces are for escaping the braces)
                Some(format!("{}='{{{{{}}}}}',", variable, variable))
            } else {
                None
            }
        })
        .collect()
}
