use fiberplane_models::formatting::{Annotation, AnnotationWithOffset};
use fiberplane_models::notebooks::*;
use fiberplane_models::timestamps::{NewTimeRange, RelativeTimeRange};
use fiberplane_templates::*;

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
