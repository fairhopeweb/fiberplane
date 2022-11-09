/*
 * Fiberplane API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnnotationType {
    #[serde(rename = "start_bold")]
    StartBold,
    #[serde(rename = "end_bold")]
    EndBold,
    #[serde(rename = "start_code")]
    StartCode,
    #[serde(rename = "end_code")]
    EndCode,
    #[serde(rename = "start_highlight")]
    StartHighlight,
    #[serde(rename = "end_highlight")]
    EndHighlight,
    #[serde(rename = "start_italics")]
    StartItalics,
    #[serde(rename = "end_italics")]
    EndItalics,
    #[serde(rename = "start_link")]
    StartLink,
    #[serde(rename = "end_link")]
    EndLink,
    #[serde(rename = "mention")]
    Mention,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "start_strikethrough")]
    StartStrikethrough,
    #[serde(rename = "end_strikethrough")]
    EndStrikethrough,
    #[serde(rename = "start_underline")]
    StartUnderline,
    #[serde(rename = "end_underline")]
    EndUnderline,
}

impl ToString for AnnotationType {
    fn to_string(&self) -> String {
        match self {
            Self::StartBold => String::from("start_bold"),
            Self::EndBold => String::from("end_bold"),
            Self::StartCode => String::from("start_code"),
            Self::EndCode => String::from("end_code"),
            Self::StartHighlight => String::from("start_highlight"),
            Self::EndHighlight => String::from("end_highlight"),
            Self::StartItalics => String::from("start_italics"),
            Self::EndItalics => String::from("end_italics"),
            Self::StartLink => String::from("start_link"),
            Self::EndLink => String::from("end_link"),
            Self::Mention => String::from("mention"),
            Self::Timestamp => String::from("timestamp"),
            Self::StartStrikethrough => String::from("start_strikethrough"),
            Self::EndStrikethrough => String::from("end_strikethrough"),
            Self::StartUnderline => String::from("start_underline"),
            Self::EndUnderline => String::from("end_underline"),
        }
    }
}
