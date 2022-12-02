use fiberplane_models::data_sources::SelectedDataSource;
use fiberplane_models::formatting::{Annotation, AnnotationWithOffset};
use fiberplane_models::names::Name;
use fiberplane_models::notebooks::*;
use fiberplane_models::timestamps::{NewTimeRange, RelativeTimeRange};
use fiberplane_templates::*;
use once_cell::unsync::Lazy;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::{fs, path::PathBuf};
use time::macros::datetime;

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
    cells: vec![
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
    ],
});

#[test]
fn matches_fiberplane_rs() {
    let template = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/template.jsonnet"),
    )
    .unwrap();
    let args = HashMap::from([("incidentName", "API Outage")]);
    let actual = expand_template(&template, args).unwrap();
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
            formatting: vec![
                // This bold range intentionally overlaps with the mustache variable substitution
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 13,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 46,
                },
            ],
            ..Default::default()
        })],
        selected_data_sources: Default::default(),
        time_range: NewTimeRange::Relative(RelativeTimeRange { minutes: -60 }),
        labels: Vec::new(),
    };
    let template = notebook_to_template(notebook);
    assert!(template.contains("fmt.bold([personName, ', great to have you'])"),);
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
