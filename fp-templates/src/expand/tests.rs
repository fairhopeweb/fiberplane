use super::*;
use fiberplane::protocols::core::{
    Cell, DataSource, InlineDataSource, Label, ListItemCell, ListType, NotebookDataSource,
    PrometheusDataSource, TextCell,
};
use fiberplane::protocols::formatting::Annotation::Timestamp;
use fiberplane::protocols::formatting::{Annotation, AnnotationWithOffset, Mention};
use serde_json::{json, Map};
use std::collections::HashMap;
use std::iter::FromIterator;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[test]
fn expands_without_top_level_function() {
    let template = "{title: 'hello'}";
    let expander = TemplateExpander::default();
    let output = expander
        .expand_template_to_string(
            template,
            [("not used".to_string(), Value::String("value".to_string()))],
            false,
        )
        .unwrap();
    assert_eq!(output, "{\"title\": \"hello\"}");
}

#[test]
fn includes_timestamp_by_default() {
    let template = "{time: std.extVar('UNIX_TIMESTAMP')}";
    let expander = TemplateExpander::default();
    // This will panic if the value is not set
    expander
        .expand_template_to_string(template, EMPTY_ARGS, false)
        .unwrap();
}

#[test]
fn allows_setting_custom_timestamp() {
    let template = "{time: std.extVar('UNIX_TIMESTAMP')}";
    let mut expander = TemplateExpander::default();
    expander.set_unix_timestamp(0.0);
    let output = expander
        .expand_template_to_string(template, EMPTY_ARGS, false)
        .unwrap();
    assert_eq!(output, "{\"time\": 0}");
}

#[test]
fn includes_data_sources_by_default() {
    let template = "{dataSources: std.extVar('PROXY_DATA_SOURCES')}";
    let expander = TemplateExpander::default();
    // This will panic if the value is not set
    expander
        .expand_template_to_string(template, EMPTY_ARGS, false)
        .unwrap();
}

#[test]
fn accepts_map_or_hashmap() {
    let template = "function(title) { title: title }";
    let expander = TemplateExpander::default();
    let args: HashMap<_, _> =
        HashMap::from_iter([("title", Value::String("my title".to_string()))]);
    let output = expander
        .expand_template_to_string(template, args, false)
        .unwrap();
    assert_eq!(output, "{\"title\": \"my title\"}");

    let args = Map::from_iter([(
        "title".to_string(),
        Value::String("other title".to_string()),
    )]);
    let output = expander
        .expand_template_to_string(template, args, false)
        .unwrap();
    assert_eq!(output, "{\"title\": \"other title\"}");
}

#[test]
fn accepts_non_value_args() {
    let template = "function(title) { title: title }";
    let expander = TemplateExpander::default();
    let args: HashMap<_, _> = HashMap::from_iter([("title", "my title".to_string())]);
    let output = expander
        .expand_template_to_string(template, args, false)
        .unwrap();
    assert_eq!(output, "{\"title\": \"my title\"}");
}

#[test]
fn expands_if_tlas_not_used() {
    let template = "function(title='hello') {title: title}";
    let expander = TemplateExpander::default();
    let output = expander
        .expand_template_to_string(
            template,
            [
                ("not used".to_string(), Value::String("value".to_string())),
                ("title".to_string(), Value::String("okay".to_string())),
            ],
            false,
        )
        .unwrap();
    assert_eq!(output, "{\"title\": \"okay\"}");
}

#[test]
fn expands_nested_lists() {
    let template = "local fp = import 'fiberplane.libsonnet';
    fp.notebook.new('a')
        .addCells([
            fp.cell.orderedList([ 'A', ['1', '2', ['i', 'ii']] ])
        ])";
    let output = expand_template(template, EMPTY_ARGS).unwrap();
    let cells: Vec<ListItemCell> = output
        .cells
        .into_iter()
        .map(|c| {
            if let Cell::ListItem(c) = c {
                c
            } else {
                panic!("Expected ListItem")
            }
        })
        .collect();
    assert_eq!(
        cells,
        &[
            ListItemCell {
                id: "1".to_string(),
                content: "A".to_string(),
                list_type: ListType::Ordered,
                level: None,
                read_only: None,
                start_number: Some(1),
                formatting: None,
            },
            ListItemCell {
                id: "2".to_string(),
                content: "1".to_string(),
                list_type: ListType::Ordered,
                level: Some(1),
                read_only: None,
                start_number: Some(1),
                formatting: None,
            },
            ListItemCell {
                id: "3".to_string(),
                content: "2".to_string(),
                list_type: ListType::Ordered,
                level: Some(1),
                read_only: None,
                start_number: Some(2),
                formatting: None,
            },
            ListItemCell {
                id: "4".to_string(),
                content: "i".to_string(),
                list_type: ListType::Ordered,
                level: Some(2),
                read_only: None,
                start_number: Some(1),
                formatting: None,
            },
            ListItemCell {
                id: "5".to_string(),
                content: "ii".to_string(),
                list_type: ListType::Ordered,
                level: Some(2),
                read_only: None,
                start_number: Some(2),
                formatting: None,
            }
        ]
    );
}

#[test]
fn filters_out_invalid_labels() {
    let template = "local fp = import 'fiberplane.libsonnet';
    fp.notebook.new('title').addLabels({'a': 'b', '-invalidkey': 'c', 'd': '\ninvalidvalue', 'e': 'f'})";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    assert_eq!(
        notebook.labels,
        &[
            Label {
                key: "a".to_string(),
                value: "b".to_string(),
            },
            Label {
                key: "e".to_string(),
                value: "f".to_string(),
            }
        ]
    );
}

#[test]
fn errors_include_line_numbers() {
    let template = "local a = '';
invalid!";
    let args: [(&str, Value); 0] = [];
    match expand_template(template, args) {
        Ok(_) => panic!("should have errored"),
        Err(Error::Evaluation(err)) => assert!(err.contains("template:2:8")),
        Err(_) => panic!("wrong error"),
    }
}

#[test]
fn returns_helpful_error_if_missing_argument() {
    let template = "local fp = import 'fiberplane.libsonnet';
    function(title)
    fp.notebook.new(title)";
    let args: [(&str, Value); 0] = [];
    match expand_template(template, args) {
        Ok(_) => panic!("Should have errored"),
        Err(Error::MissingArgument(parameter)) => assert_eq!(parameter, "title"),
        Err(err) => panic!("wrong error: {:?}", err),
    }
}

#[test]
fn extract_template_parameters_non_function() {
    let template = "local fp = import 'fiberplane.libsonnet';
    fp.notebook.new(title)";
    let parameters = extract_template_parameters(template).unwrap();
    assert!(parameters.is_empty());
}

#[test]
fn extract_template_parameters_no_parameters() {
    let template = "function() {}";
    let parameters = extract_template_parameters(template).unwrap();
    assert!(parameters.is_empty());
}

#[test]
fn extract_template_parameters_required() {
    let template = "function(requiredParam1, requiredParam2) { title: 'my title' }";
    let params = extract_template_parameters(template).unwrap();
    assert_eq!(params.len(), 2);
    assert_eq!(
        params[0],
        TemplateParameter {
            name: "requiredParam1".to_string(),
            ty: TemplateParameterType::Unknown,
            default_value: None
        }
    );
    assert_eq!(
        params[1],
        TemplateParameter {
            name: "requiredParam2".to_string(),
            ty: TemplateParameterType::Unknown,
            default_value: None
        }
    );
}

#[test]
fn extract_template_parameters_optional() {
    let template = "function(
        optionalString='test',
        optionalNumber=1,
        optionalBoolean=true,
        optionalObject={},
        optionalArray=[],
        optionalNull=null,
    ) { title: 'my title' }";
    let params = extract_template_parameters(template).unwrap();
    assert_eq!(params.len(), 6);
    assert_eq!(
        params[0],
        TemplateParameter {
            name: "optionalString".to_string(),
            ty: TemplateParameterType::String,
            default_value: Some(Value::String("test".to_string()))
        }
    );
    assert_eq!(
        params[1],
        TemplateParameter {
            name: "optionalNumber".to_string(),
            ty: TemplateParameterType::Number,
            default_value: Some(Value::Number(Number::from_f64(1.0).unwrap()))
        }
    );
    assert_eq!(
        params[2],
        TemplateParameter {
            name: "optionalBoolean".to_string(),
            ty: TemplateParameterType::Boolean,
            default_value: Some(Value::Bool(true))
        }
    );
    assert_eq!(
        params[3],
        TemplateParameter {
            name: "optionalObject".to_string(),
            ty: TemplateParameterType::Object,
            default_value: Some(Value::Object(Map::new()))
        }
    );
    assert_eq!(
        params[4],
        TemplateParameter {
            name: "optionalArray".to_string(),
            ty: TemplateParameterType::Array,
            default_value: Some(Value::Array(vec![]))
        }
    );
    assert_eq!(
        params[5],
        TemplateParameter {
            name: "optionalNull".to_string(),
            ty: TemplateParameterType::Unknown,
            default_value: Some(Value::Null)
        }
    );
}

#[test]
fn extract_template_parameters_ignores_non_serializable_types() {
    let template = "function(optionalObject={ a: function() {}, b: 2, c: 'three'}) {}";
    let params = extract_template_parameters(template).unwrap();
    assert_eq!(
        params[0],
        TemplateParameter {
            name: "optionalObject".to_string(),
            ty: TemplateParameterType::Object,
            default_value: Some(json!({
                "b": 2.0,
                "c": "three"
            }))
        }
    );
}

#[test]
fn extract_template_parameters_value_from_context() {
    let template = "local a = 1;
    local b(x) = x + 1;
    local c = 'three';
    function(x=a, y=b(1), z=c) {}";
    let params = extract_template_parameters(template).unwrap();
    assert_eq!(
        params,
        vec![
            TemplateParameter {
                name: "x".to_string(),
                ty: TemplateParameterType::Number,
                default_value: Some(Value::Number(Number::from_f64(1.0).unwrap()))
            },
            TemplateParameter {
                name: "y".to_string(),
                ty: TemplateParameterType::Number,
                default_value: Some(Value::Number(Number::from_f64(2.0).unwrap()))
            },
            TemplateParameter {
                name: "z".to_string(),
                ty: TemplateParameterType::String,
                default_value: Some(Value::String("three".to_string()))
            }
        ]
    );
}

#[test]
fn formatting_basic() {
    let template = "local fp = import 'fiberplane.libsonnet';
        local fmt = fp.format;

        fp.notebook.new('title')
        .setTimeRangeRelative(60)
        .addCells(
        [
            fp.cell.text(fmt.bold('some bold text')),
            fp.cell.text(fmt.code('some code')),
            fp.cell.text(fmt.highlight('some highlighted text')),
            fp.cell.text(fmt.italics('some italicized text')),
            fp.cell.text(fmt.link('Fiberplane', 'https://fiberplane.com')),
            fp.cell.text(fmt.strikethrough('some strikethrough text')),
            fp.cell.text(fmt.underline('some underlined text')),
            fp.cell.text(fmt.mention('Bob Bobsen', 'Bob')),
            fp.cell.text(fmt.timestamp('2020-01-01T00:00:00Z')),
            fp.cell.text(fmt.label('foo', 'bar')),
            fp.cell.text(fmt.label('foo')),
        ])";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    let cells: Vec<TextCell> = notebook
        .cells
        .into_iter()
        .map(|cell| {
            if let Cell::Text(text) = cell {
                text
            } else {
                panic!("expected text cell");
            }
        })
        .collect();
    assert_eq!(cells.len(), 11);

    assert_eq!(cells[0].content, "some bold text");
    assert_eq!(
        cells[0].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartBold,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndBold,
                offset: 14,
            },
        ]
    );
    assert_eq!(cells[1].content, "some code");
    assert_eq!(
        cells[1].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartCode,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndCode,
                offset: 9,
            },
        ]
    );
    assert_eq!(cells[2].content, "some highlighted text");
    assert_eq!(
        cells[2].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartHighlight,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndHighlight,
                offset: 21,
            },
        ]
    );
    assert_eq!(cells[3].content, "some italicized text");
    assert_eq!(
        cells[3].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartItalics,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndItalics,
                offset: 20,
            },
        ]
    );
    assert_eq!(cells[4].content, "Fiberplane");
    assert_eq!(
        cells[4].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartLink {
                    url: "https://fiberplane.com".to_string()
                },
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndLink,
                offset: 10,
            },
        ]
    );
    assert_eq!(cells[5].content, "some strikethrough text");
    assert_eq!(
        cells[5].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartStrikethrough,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndStrikethrough,
                offset: 23,
            },
        ]
    );
    assert_eq!(cells[6].content, "some underlined text");
    assert_eq!(
        cells[6].formatting.as_ref().unwrap(),
        &[
            AnnotationWithOffset {
                annotation: Annotation::StartUnderline,
                offset: 0,
            },
            AnnotationWithOffset {
                annotation: Annotation::EndUnderline,
                offset: 20,
            },
        ]
    );

    assert_eq!(cells[7].content, "@Bob");
    assert_eq!(
        cells[7].formatting.as_ref().unwrap(),
        &[AnnotationWithOffset {
            annotation: Annotation::Mention(Mention {
                name: "Bob Bobsen".to_owned(),
                user_id: "Bob".to_owned(),
            }),
            offset: 0,
        }]
    );

    assert_eq!(cells[8].content, "2020-01-01T00:00:00Z");
    assert_eq!(
        cells[8].formatting.as_ref().unwrap(),
        &[AnnotationWithOffset {
            annotation: Timestamp {
                timestamp: OffsetDateTime::parse("2020-01-01T00:00:00Z", &Rfc3339).unwrap()
            },
            offset: 0,
        }]
    );

    assert_eq!(cells[9].content, "foo:bar");
    assert_eq!(
        cells[9].formatting.as_ref().unwrap(),
        &[AnnotationWithOffset {
            annotation: Annotation::Label(Label {
                key: "foo".to_owned(),
                value: "bar".to_owned(),
            }),
            offset: 0,
        }]
    );

    assert_eq!(cells[10].content, "foo");
    assert_eq!(
        cells[10].formatting.as_ref().unwrap(),
        &[AnnotationWithOffset {
            annotation: Annotation::Label(Label {
                key: "foo".to_owned(),
                value: "".to_owned(),
            }),
            offset: 0,
        }]
    );
}

#[test]
fn formatting_nested() {
    let template = "local fp = import 'fiberplane.libsonnet';
        local fmt = fp.format;

        fp.notebook.new('title')
        .setTimeRangeRelative(60)
        .addCell(
            fp.cell.text(['some normal, ', fmt.bold(['some bold, ', fmt.italics('and some bold italicized')]), ' text'])
        )";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    if let Cell::Text(cell) = &notebook.cells[0] {
        assert_eq!(
            cell.content,
            "some normal, some bold, and some bold italicized text"
        );
        assert_eq!(
            cell.formatting.as_ref().unwrap(),
            &[
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 13,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartItalics,
                    offset: 24,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndItalics,
                    offset: 48,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 48,
                },
            ]
        );
    } else {
        panic!("expected text cell");
    }
}

#[test]
fn formatting_builder() {
    let template = "local fp = import 'fiberplane.libsonnet';
        local fmt = fp.format;

        fp.notebook.new('title')
        .setTimeRangeRelative(60)
        .addCell(
            fp.cell.text(fmt.raw('some normal, ').bold('some bold, ').italics('and some italicized').raw(' text'))
        )";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    if let Cell::Text(cell) = &notebook.cells[0] {
        assert_eq!(
            cell.content,
            "some normal, some bold, and some italicized text"
        );
        assert_eq!(
            cell.formatting.as_ref().unwrap(),
            &[
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 13,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 24,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartItalics,
                    offset: 24,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndItalics,
                    offset: 43,
                },
            ]
        );
    } else {
        panic!("expected text cell");
    }
}

#[test]
fn format_list() {
    let template = "local fp = import 'fiberplane.libsonnet';
        local c = fp.cell;
        local fmt = fp.format;

        fp.notebook.new('title')
        .setTimeRangeRelative(60)
        .addCells([
            c.orderedList([
                fmt.raw('some normal, ').bold('and some bold'),
                fmt.bold('bold item'),
                'normal item',
            ])
        ])";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    if let Cell::ListItem(cell) = &notebook.cells[0] {
        assert_eq!(cell.content, "some normal, and some bold");
        assert_eq!(
            cell.formatting.as_ref().unwrap(),
            &[
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 13,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 26,
                }
            ]
        );
    } else {
        panic!("wrong cell type")
    }
    if let Cell::ListItem(cell) = &notebook.cells[1] {
        assert_eq!(cell.content, "bold item");
        assert_eq!(
            cell.formatting.as_ref().unwrap(),
            &[
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 0,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 9,
                }
            ]
        );
    } else {
        panic!("wrong cell type")
    }
    if let Cell::ListItem(cell) = &notebook.cells[2] {
        assert_eq!(cell.content, "normal item");
        assert_eq!(cell.formatting, None);
    } else {
        panic!("wrong cell type");
    }
}

#[test]
fn add_direct_data_source() {
    let template = "local fp = import 'fiberplane.libsonnet';

        fp.notebook.new('title')
        .addDirectDataSource('data source', 'prometheus', { url: 'http://localhost:9090/api/v1/query' })";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    assert_eq!(
        notebook.data_sources.get("data source").unwrap(),
        &NotebookDataSource::Inline(InlineDataSource {
            data_source: DataSource::Prometheus(PrometheusDataSource {
                url: "http://localhost:9090/api/v1/query".to_string()
            })
        })
    );
}

#[test]
fn add_direct_data_source_with_old_signature() {
    let template = "local fp = import 'fiberplane.libsonnet';

        fp.notebook.new('title')
        .addDirectDataSource('data source', 'prometheus', 'http://localhost:9090/api/v1/query')";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    assert_eq!(
        notebook.data_sources.get("data source").unwrap(),
        &NotebookDataSource::Inline(InlineDataSource {
            data_source: DataSource::Prometheus(PrometheusDataSource {
                url: "http://localhost:9090/api/v1/query".to_string()
            })
        })
    );
}

#[test]
fn add_direct_data_source_with_old_signature_named_parameter() {
    let template = "local fp = import 'fiberplane.libsonnet';

        fp.notebook.new('title')
        .addDirectDataSource('data source', 'prometheus', url = 'http://localhost:9090/api/v1/query')";
    let notebook = expand_template(template, EMPTY_ARGS).unwrap();
    assert_eq!(
        notebook.data_sources.get("data source").unwrap(),
        &NotebookDataSource::Inline(InlineDataSource {
            data_source: DataSource::Prometheus(PrometheusDataSource {
                url: "http://localhost:9090/api/v1/query".to_string()
            })
        })
    );
}
