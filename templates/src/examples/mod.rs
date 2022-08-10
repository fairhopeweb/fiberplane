#[cfg(test)]
mod tests;

pub static INCIDENT_RESPONSE: &str =
    include_str!("../../examples/incident-response/template.jsonnet");
pub static INCIDENT_ANALYSIS: &str =
    include_str!("../../examples/incident-analysis/template.jsonnet");
pub static ROOT_CAUSE_ANALYSIS: &str =
    include_str!("../../examples/root-cause-analysis/template.jsonnet");
pub static MEETING_NOTES: &str = include_str!("../../examples/meeting-notes/template.jsonnet");
