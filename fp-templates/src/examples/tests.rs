use super::{INCIDENT_ANALYSIS, INCIDENT_RESPONSE, MEETING_NOTES, ROOT_CAUSE_ANALYSIS};
use crate::{expand_template, TemplateExpander, EMPTY_ARGS};
use fiberplane::protocols::core::{Cell, Label};
use serde_json::Value;

#[test]
pub fn no_required_arguments() {
    let expander = TemplateExpander::default();

    for template in [
        INCIDENT_RESPONSE,
        INCIDENT_ANALYSIS,
        ROOT_CAUSE_ANALYSIS,
        MEETING_NOTES,
    ] {
        // Extract the template parameters
        for param in expander.extract_template_parameters(template).unwrap() {
            assert!(param.default_value.is_some());
        }

        // Also try to expand the template with no arguments
        expander.expand_template(template, EMPTY_ARGS).unwrap();
    }
}

#[test]
pub fn incident_response() {
    let notebook = expand_template(
        INCIDENT_RESPONSE,
        [
            ("alertTitle", Value::String("Some incident".to_string())),
            (
                "alertSummary",
                Value::String("Somethin's happenin' in here".to_string()),
            ),
            ("alertSeverity", Value::String("High".to_string())),
        ],
    )
    .unwrap();

    assert_eq!(notebook.title, "Some incident");
    if let Cell::Heading(cell) = &notebook.cells[2] {
        assert_eq!(cell.content, "Severity: High");
    } else {
        panic!("Expected heading cell");
    }
    if let Cell::Text(cell) = &notebook.cells[4] {
        assert_eq!(cell.content, "Somethin's happenin' in here");
    } else {
        panic!("Expected text cell");
    }
}

#[test]
pub fn incident_analysis() {
    let notebook = expand_template(
        INCIDENT_ANALYSIS,
        [
            ("incidentNumber".to_string(), Value::Number(123i32.into())),
            (
                "incidentTitle".to_string(),
                Value::String("Some Title".to_string()),
            ),
            ("serviceName".to_string(), Value::String("API".to_string())),
            (
                "environmentName".to_string(),
                Value::String("Production".to_string()),
            ),
        ],
    )
    .unwrap();

    assert_eq!(notebook.title, "Incident Analysis: 123 - Some Title");
    assert_eq!(
        notebook.labels,
        &[
            Label {
                key: "environment".to_string(),
                value: "Production".to_string(),
            },
            Label {
                key: "service".to_string(),
                value: "API".to_string(),
            },
            Label {
                key: "type".to_string(),
                value: "incident-analysis".to_string(),
            },
        ]
    );
}

#[test]
pub fn root_cause_analysis() {
    let notebook = expand_template(
        ROOT_CAUSE_ANALYSIS,
        [
            (
                "incidentTitle".to_string(),
                Value::String("API Outage".to_string()),
            ),
            (
                "incidentNumber".to_string(),
                Value::String("123".to_string()),
            ),
            (
                "incidentCommander".to_string(),
                Value::String("Princess Leia".to_string()),
            ),
        ],
    )
    .unwrap();

    assert_eq!(notebook.title, "RCA for Incident: API Outage 123");
    if let Cell::Text(cell) = &notebook.cells[0] {
        assert_eq!(cell.content, "Incident Commander: Princess Leia");
    } else {
        panic!("Expected text cell");
    }
}

#[test]
pub fn meeting_notes() {
    let notebook = expand_template(
        MEETING_NOTES,
        [
            (
                "topic".to_string(),
                Value::String("A Very Important Project".to_string()),
            ),
            ("date".to_string(), Value::String("1/1/1970".to_string())),
        ],
    )
    .unwrap();
    assert_eq!(
        notebook.title,
        "Meeting Notes: A Very Important Project - 1/1/1970"
    );
}
