use assert_json_diff::{assert_json_eq, assert_json_include};
use fiberplane::protocols::core::{
    Cell, CheckboxCell, CodeCell, DataSource, DataSourceType, ElasticsearchDataSource, HeadingCell,
    HeadingType, ImageCell, InlineDataSource, Label, ListItemCell, ListType, LokiCell,
    LokiDataSource, NewNotebook, NotebookDataSource, ProxyDataSource, TextCell, TimeRange,
};
use fiberplane::protocols::formatting::{Annotation, AnnotationWithOffset};
use fp_templates::{
    expand_template, extract_template_parameters, notebook_to_template, TemplateExpander,
    TemplateParameter, TemplateParameterType, EMPTY_ARGS,
};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref NOTEBOOK: NewNotebook = NewNotebook {
        title: "Incident: 'API Outage'".to_string(),
        time_range: TimeRange {
            from: 1639239669.739,
            to: 1639239729.0,
        },
        data_sources: BTreeMap::from([
            (
                "direct elasticsearch".to_string(),
                NotebookDataSource::Inline(InlineDataSource {
                    data_source: DataSource::Elasticsearch(ElasticsearchDataSource {
                        url: "https://elasticsearch.dev.fiberplane.io".to_string(),
                        timestamp_field_names: vec!["@timestamp".to_string()],
                        body_field_names: vec!["message".to_string()],
                    })
                })
            ),
            (
                "direct loki".to_string(),
                NotebookDataSource::Inline(InlineDataSource {
                    data_source: DataSource::Loki(LokiDataSource {
                        url: "https://loki.dev.fiberplane.io".to_string()
                    })
                })
            ),
        ]),
        cells: vec![
            Cell::Text(TextCell {
                id: "1".to_string(),
                content: "Let's debug this incident! foo:bar baz".to_string(),
                formatting: Some(vec![
                    AnnotationWithOffset {
                        annotation: Annotation::StartItalics,
                        offset: 6
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndItalics,
                        offset: 11
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::StartBold,
                        offset: 17
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndBold,
                        offset: 26
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::Label(Label {
                            key: "foo".to_string(),
                            value: "bar".to_string()
                        }),
                        offset: 27
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::Label(Label {
                            key: "baz".to_string(),
                            value: "".to_string()
                        }),
                        offset: 35
                    }
                ]),
                ..Default::default()
            }),
            Cell::Heading(HeadingCell {
                id: "2".to_string(),
                content: "TODOs:".to_string(),
                formatting: None,
                heading_type: HeadingType::H2,
                read_only: Some(true),
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
                formatting: None,
                heading_type: HeadingType::H2,
                read_only: Some(true),
            }),
            Cell::Loki(LokiCell {
                id: "8".to_string(),
                content: "loki query".to_string(),
                read_only: None,
            }),
            Cell::ListItem(ListItemCell {
                id: "9".to_string(),
                content: "Step 1".to_string(),
                formatting: None,
                list_type: ListType::Ordered,
                level: None,
                start_number: Some(1),
                read_only: None,
            }),
            Cell::Code(CodeCell {
                id: "10".to_string(),
                content: "Some code".to_string(),
                read_only: None,
                syntax: None,
            }),
            Cell::ListItem(ListItemCell {
                id: "11".to_string(),
                content: "Step 2".to_string(),
                formatting: None,
                list_type: ListType::Ordered,
                level: None,
                start_number: Some(2),
                read_only: None,
            }),
            Cell::ListItem(ListItemCell {
                id: "12".to_string(),
                content: "Bullet 1".to_string(),
                formatting: None,
                list_type: ListType::Unordered,
                level: Some(1),
                start_number: Some(1),
                read_only: None,
            }),
            Cell::ListItem(ListItemCell {
                id: "13".to_string(),
                content: "Bullet 2".to_string(),
                formatting: None,
                list_type: ListType::Unordered,
                level: Some(1),
                start_number: Some(2),
                read_only: None,
            }),
            Cell::Image(ImageCell {
                id: "14".to_string(),
                url: Some("http://example.com/image.png".to_string()),
                file_id: None,
                width: None,
                height: None,
                preview: None,
                progress: None,
                read_only: None,
            })
        ],
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
        ]
    };
    static ref NOTEBOOK_JSON: Value = serde_json::to_value(&*NOTEBOOK).unwrap();
}

#[test]
fn matches_fiberplane_rs() {
    let template = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/template.jsonnet"),
    )
    .unwrap();
    let args = HashMap::from([("incidentName", "API Outage")]);
    let output = expand_template(&template, args).unwrap();
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
    assert_json_eq!(output, &*NOTEBOOK);
}

#[test]
fn export_notebook_to_template_and_back() {
    let template = notebook_to_template(NOTEBOOK.clone());
    println!("{}", template);
    let actual = expand_template(template, EMPTY_ARGS).unwrap();

    assert_eq!(actual.time_range.to - actual.time_range.from, 60.0);

    // Ignore the time range here because it's been updated to a range with the
    // same duration but ending at the moment the notebook was created
    let mut expected = serde_json::to_value(&*NOTEBOOK).unwrap();
    if let Some(object) = expected.as_object_mut() {
        object.remove("timeRange").unwrap();
    }
    assert_json_include!(actual: actual, expected: expected);
}

#[test]
fn searches_proxy_data_sources() {
    let data_sources = json!([{
        "name": "Production Prometheus",
        "type": "prometheus",
        "proxy": {
            "id": "8ac70ba9-e7d3-4ebb-8520-884beb9d5d50",
            "name": "production"
        }
    }]);
    let notebook_data_source = NotebookDataSource::Inline(InlineDataSource {
        data_source: DataSource::Proxy(ProxyDataSource {
            data_source_name: "Production Prometheus".to_string(),
            proxy_id: "8ac70ba9-e7d3-4ebb-8520-884beb9d5d50".to_string(),
            data_source_type: DataSourceType::Prometheus,
        }),
    });
    let mut expander = TemplateExpander::default();
    expander.set_proxy_data_sources(serde_json::to_value(&data_sources).unwrap());

    let template = |query| {
        format!(
            "local fp = import 'fiberplane.libsonnet';
    fp.notebook.new('title')
    .setTimeRangeRelative(60)
    .addProxyDataSource({})",
            query
        )
    };

    // Search by type
    let notebook = expander
        .expand_template(template("type='prometheus'"), EMPTY_ARGS)
        .unwrap();
    assert_eq!(
        notebook.data_sources.get("Production Prometheus").unwrap(),
        &notebook_data_source
    );

    // Search by data source and proxy name
    let notebook = expander
        .expand_template(
            template("proxyName='production', name='Production Prometheus'"),
            EMPTY_ARGS,
        )
        .unwrap();
    assert_eq!(
        notebook.data_sources.get("Production Prometheus").unwrap(),
        &notebook_data_source
    );
}

#[test]
fn mustache_substitution_in_title() {
    let notebook = NewNotebook {
        title: r#"Hello {{personName}}, this is a {{notebookCategory}}"#.to_string(),
        cells: Vec::new(),
        time_range: TimeRange {
            from: 0.0,
            to: 60.0,
        },
        data_sources: BTreeMap::new(),
        labels: Vec::new(),
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
fn mustache_substitution_with_formatting() {
    let notebook = NewNotebook {
        title: r#"Test"#.to_string(),
        cells: vec![Cell::Text(TextCell {
            id: "1".to_string(),
            content: r#"{{greeting}} {{personName}}, great to have you"#.to_string(),
            formatting: Some(vec![
                // This bold range intentionally overlaps with the mustache variable substitution
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 13,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 46,
                },
            ]),
            read_only: None,
        })],
        time_range: TimeRange {
            from: 0.0,
            to: 60.0,
        },
        data_sources: BTreeMap::new(),
        labels: Vec::new(),
    };
    let template = notebook_to_template(notebook);
    assert!(template
        .find("fmt.bold([personName, ', great to have you'])")
        .is_some(),);
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
        time_range: TimeRange {
            from: 0.0,
            to: 60.0,
        },
        data_sources: BTreeMap::new(),
        labels: Vec::new(),
    };
    let template = notebook_to_template(notebook);
    let params = extract_template_parameters(&template).unwrap();
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
