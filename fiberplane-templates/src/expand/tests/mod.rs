use super::*;
use crate::types::{TemplateParameter, TemplateParameterType};
use crate::*;
use fiberplane_models::data_sources::SelectedDataSource;
use fiberplane_models::formatting::Annotation::{self, Timestamp};
use fiberplane_models::formatting::{AnnotationWithOffset, Formatting, Mention};
use fiberplane_models::names::Name;
use fiberplane_models::notebooks::*;
use fiberplane_models::timestamps::{NewTimeRange, RelativeTimeRange};
use once_cell::unsync::Lazy;
use pretty_assertions::assert_eq;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;
use time::format_description::well_known::Rfc3339;
use time::macros::datetime;
use time::OffsetDateTime;

const CELLS: Lazy<Vec<Cell>> = Lazy::new(|| {
    vec![
        Cell::Text(TextCell {
            id: "1".to_string(),
            content: "Let's debug this incident! foo:bar baz".to_string(),
            formatting: vec![
                AnnotationWithOffset {
                    annotation: Annotation::StartItalics,
                    offset: 6,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndItalics,
                    offset: 11,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 17,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 26,
                },
                AnnotationWithOffset {
                    annotation: Annotation::Label(Label {
                        key: "foo".to_string(),
                        value: "bar".to_string(),
                    }),
                    offset: 27,
                },
                AnnotationWithOffset {
                    annotation: Annotation::Label(Label {
                        key: "baz".to_string(),
                        value: "".to_string(),
                    }),
                    offset: 35,
                },
            ],
            ..Default::default()
        }),
        Cell::Heading(HeadingCell {
            id: "2".to_string(),
            content: "TODOs:".to_string(),
            heading_type: HeadingType::H2,
            read_only: Some(true),
            ..Default::default()
        }),
        Cell::Checkbox(CheckboxCell {
            id: "3".to_string(),
            content: "Investigate".to_string(),
            ..Default::default()
        }),
        Cell::Code(CodeCell {
            id: "4".to_string(),
            content: "// Some code to run
let a = 'b';
let b = \"c\";"
                .to_string(),
            ..Default::default()
        }),
        Cell::Checkbox(CheckboxCell {
            id: "5".to_string(),
            content: "Resolve".to_string(),
            ..Default::default()
        }),
        Cell::Checkbox(CheckboxCell {
            id: "6".to_string(),
            content: "Profit".to_string(),
            ..Default::default()
        }),
        Cell::Heading(HeadingCell {
            id: "7".to_string(),
            content: "Hypotheses".to_string(),
            heading_type: HeadingType::H2,
            read_only: Some(true),
            ..Default::default()
        }),
        Cell::Provider(ProviderCell {
            id: "8".to_string(),
            intent: "loki,events".to_string(),
            query_data: Some("application/x-www-form-urlencoded,query=loki+query".to_string()),
            ..Default::default()
        }),
        Cell::ListItem(ListItemCell {
            id: "9".to_string(),
            content: "Step 1".to_string(),
            list_type: ListType::Ordered,
            level: None,
            start_number: Some(1),
            read_only: None,
            ..Default::default()
        }),
        Cell::Code(CodeCell {
            id: "10".to_string(),
            content: "Some code".to_string(),
            ..Default::default()
        }),
        Cell::ListItem(ListItemCell {
            id: "11".to_string(),
            content: "Step 2".to_string(),
            list_type: ListType::Ordered,
            start_number: Some(2),
            ..Default::default()
        }),
        Cell::ListItem(ListItemCell {
            id: "12".to_string(),
            content: "Bullet 1".to_string(),
            list_type: ListType::Unordered,
            level: Some(1),
            start_number: Some(1),
            ..Default::default()
        }),
        Cell::ListItem(ListItemCell {
            id: "13".to_string(),
            content: "Bullet 2".to_string(),
            list_type: ListType::Unordered,
            level: Some(1),
            start_number: Some(2),
            ..Default::default()
        }),
        Cell::Image(ImageCell {
            id: "14".to_string(),
            url: Some("http://example.com/image.png".to_string()),
            ..Default::default()
        }),
        Cell::Provider(ProviderCell {
            id: "15".to_string(),
            intent: "prometheus,timeseries".to_string(),
            query_data: Some("application/x-www-form-urlencoded,query=http_requests".to_string()),
            title: "sample title".to_string(),
            ..Default::default()
        }),
        Cell::Provider(ProviderCell {
            id: "16".to_string(),
            intent: "prometheus,timeseries".to_string(),
            ..Default::default()
        }),
        Cell::Text(TextCell {
            id: "17".to_string(),
            content: "Prefix: 2022-10-24T10:42:10.977Z - error triggered".to_string(),
            formatting: vec![AnnotationWithOffset {
                offset: 8,
                annotation: Annotation::Timestamp {
                    timestamp: datetime!(2022-10-24 10:42:10.977 UTC),
                },
            }],
            ..Default::default()
        }),
    ]
});
const NOTEBOOK: Lazy<NewNotebook> = Lazy::new(|| NewNotebook {
    title: "Incident: 'API Outage'".to_string(),
    time_range: NewTimeRange::Relative(RelativeTimeRange { minutes: -60 }),
    selected_data_sources: BTreeMap::from_iter([(
        "prometheus".to_string(),
        SelectedDataSource {
            name: Name::from_static("prometheus"),
            proxy_name: Some(Name::from_static("dev")),
        },
    )]),
    cells: CELLS.clone(),
    labels: vec![
        Label {
            key: "key1".to_string(),
            value: "".to_string(),
        },
        Label {
            key: "key2".to_string(),
            value: "value2".to_string(),
        },
        Label {
            key: "key3".to_string(),
            value: "".to_string(),
        },
        Label {
            key: "key4".to_string(),
            value: "value4".to_string(),
        },
        Label {
            key: "key5".to_string(),
            value: "".to_string(),
        },
    ],
    front_matter: FrontMatter::new(),
});

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
                start_number: Some(1),
                ..Default::default()
            },
            ListItemCell {
                id: "2".to_string(),
                content: "1".to_string(),
                list_type: ListType::Ordered,
                level: Some(1),
                start_number: Some(1),
                ..Default::default()
            },
            ListItemCell {
                id: "3".to_string(),
                content: "2".to_string(),
                list_type: ListType::Ordered,
                level: Some(1),
                start_number: Some(2),
                ..Default::default()
            },
            ListItemCell {
                id: "4".to_string(),
                content: "i".to_string(),
                list_type: ListType::Ordered,
                level: Some(2),
                start_number: Some(1),
                ..Default::default()
            },
            ListItemCell {
                id: "5".to_string(),
                content: "ii".to_string(),
                list_type: ListType::Ordered,
                level: Some(2),
                start_number: Some(2),
                ..Default::default()
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
        Err(err) => panic!("wrong error: {err:?}"),
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
        &cells[0].formatting,
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
        &cells[1].formatting,
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
        &cells[2].formatting,
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
        &cells[3].formatting,
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
        &cells[4].formatting,
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
        &cells[5].formatting,
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
        &cells[6].formatting,
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
        &cells[7].formatting,
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
        &cells[8].formatting,
        &[AnnotationWithOffset {
            annotation: Timestamp {
                timestamp: OffsetDateTime::parse("2020-01-01T00:00:00Z", &Rfc3339).unwrap()
            },
            offset: 0,
        }]
    );

    assert_eq!(cells[9].content, "foo:bar");
    assert_eq!(
        &cells[9].formatting,
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
        &cells[10].formatting,
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
            &cell.formatting,
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
            &cell.formatting,
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
            &cell.formatting,
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
            &cell.formatting,
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
        assert_eq!(cell.formatting, Formatting::default());
    } else {
        panic!("wrong cell type");
    }
}

#[test]
fn matches_fiberplane_rs() {
    let template = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/expand/tests/assets/template.jsonnet"),
    )
    .unwrap();
    let args = HashMap::from([("incidentName", "API Outage")]);
    let actual = expand_template(template, args).unwrap();
    assert_eq!(actual, *NOTEBOOK);
}

#[test]
fn export_notebook_to_template_and_back() {
    let template = notebook_to_template(NOTEBOOK.clone());
    let actual = expand_template(template, EMPTY_ARGS).unwrap();
    assert_eq!(actual, *NOTEBOOK);
}

#[test]
fn mustache_substitution_in_title() {
    let notebook = NewNotebook {
        title: r#"Hello {{personName}}, this is a {{notebookCategory}}"#.to_string(),
        cells: Vec::new(),
        selected_data_sources: Default::default(),
        time_range: NewTimeRange::Relative(RelativeTimeRange { minutes: -60 }),
        labels: Vec::new(),
        front_matter: FrontMatter::new(),
    };
    let template = notebook_to_template(notebook);
    let notebook = expand_template(
        template,
        [
            ("personName", Value::String("Bob".to_string())),
            ("notebookCategory", Value::String("Notebook".to_string())),
        ],
    )
    .unwrap();
    assert_eq!(notebook.title, "Hello Bob, this is a Notebook");
}

#[test]
fn mustache_substitution_to_function_parameters() {
    let notebook = NewNotebook {
        title: r#"Hello {{personName}}"#.to_string(),
        cells: vec![Cell::Text(TextCell {
            id: "1".to_string(),
            content: r#"{{greeting}} {{personName}}, great to have you"#.to_string(),
            ..Default::default()
        })],
        selected_data_sources: Default::default(),
        time_range: NewTimeRange::Relative(RelativeTimeRange { minutes: -60 }),
        labels: Vec::new(),
        front_matter: FrontMatter::new(),
    };
    let template = notebook_to_template(notebook);
    let params = extract_template_parameters(template).unwrap();
    // Deduplicates the `personName` parameter
    assert_eq!(params.len(), 2);
    assert_eq!(
        params[0],
        TemplateParameter {
            name: "personName".to_string(),
            default_value: Some(Value::String(r#"{{personName}}"#.to_string())),
            ty: TemplateParameterType::String,
        }
    );
    assert_eq!(
        params[1],
        TemplateParameter {
            name: "greeting".to_string(),
            default_value: Some(Value::String(r#"{{greeting}}"#.to_string())),
            ty: TemplateParameterType::String,
        }
    );
}

#[test]
fn export_cells_to_snippet_and_back() {
    let snippet = cells_to_snippet(&CELLS);
    let actual = expand_snippet(snippet).unwrap();
    assert_eq!(actual, *CELLS);
}
