use fiberplane_models::formatting::Mention;
use fiberplane_models::notebooks::{DividerCell, Label, TextCell};
use time::OffsetDateTime;

use super::*;

#[test]
fn formatting_basic() {
    let content = "some normal, some bold, and some italicized text";
    let formatting = vec![
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
    ];
    let actual = format_content(content, &formatting);
    // alternative: "fmt.raw('some normal, ').bold('some bold, ').italics('and some italicized text')"
    assert_eq!(
        actual,
        "['some normal, ', fmt.bold(['some bold, ']), fmt.italics(['and some italicized']), ' text']"
    );
}

#[test]
fn formatting_nested() {
    let content = "some normal, some bold, and some bold italicized text";
    let formatting = vec![
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
    ];
    let actual = format_content(content, &formatting);
    assert_eq!(actual, "['some normal, ', fmt.bold(['some bold, ', fmt.italics(['and some bold italicized'])]), ' text']");
}

#[test]
fn format_link() {
    let content = "see here for more";
    let formatting = vec![
        AnnotationWithOffset {
            annotation: Annotation::StartLink {
                url: "https://example.com/more".to_string(),
            },
            offset: 4,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndLink,
            offset: 8,
        },
    ];
    let actual = format_content(content, &formatting);
    assert_eq!(
        actual,
        "['see ', fmt.link(url='https://example.com/more', content=['here']), ' for more']"
    );
}

#[test]
fn format_unclosed() {
    let content = "some normal, some bold";
    let formatting = vec![AnnotationWithOffset {
        annotation: Annotation::StartBold,
        offset: 13,
    }];
    let actual = format_content(content, &formatting);
    assert_eq!(actual, "['some normal, ', fmt.bold(['some bold'])]");
}

#[test]
fn format_mention() {
    let content = "hi @Bob Bobsen mention";
    let formatting = vec![AnnotationWithOffset {
        annotation: Annotation::Mention(Mention {
            name: "Bob Bobsen".to_string(),
            user_id: "1234".to_string(),
        }),
        offset: 3,
    }];
    let actual = format_content(content, &formatting);
    assert_eq!(
        actual,
        "['hi ', fmt.mention('Bob Bobsen', '1234'), ' mention']"
    );
}

#[test]
fn format_timestamp() {
    let content = "hi 2020-01-01T00:00:00Z timestamp";
    let formatting = vec![AnnotationWithOffset {
        annotation: Annotation::Timestamp {
            timestamp: OffsetDateTime::parse("2020-01-01T00:00:00Z", &Rfc3339).unwrap(),
        },
        offset: 3,
    }];
    let actual = format_content(content, &formatting);
    assert_eq!(
        actual,
        "['hi ', fmt.timestamp('2020-01-01T00:00:00Z'), ' timestamp']"
    );
}

#[test]
fn format_label() {
    let content = "hi foo:bar label";
    let formatting = vec![AnnotationWithOffset {
        annotation: Annotation::Label(Label {
            key: "foo".to_string(),
            value: "bar".to_string(),
        }),
        offset: 3,
    }];
    let actual = format_content(content, &formatting);
    assert_eq!(actual, "['hi ', fmt.label('foo', 'bar'), ' label']");
}

#[test]
fn print_text_cell() {
    let mut writer = CodeWriter::new();
    print_cell(
        &mut writer,
        &Cell::Text(TextCell {
            id: "c1".to_owned(),
            content: "I'm a text cell".to_owned(),
            ..Default::default()
        }),
    );
    assert_eq!(writer.to_string(), "c.text(\"I'm a text cell\"),\n");
}

#[test]
fn print_divider_cell() {
    let mut writer = CodeWriter::new();
    print_cell(
        &mut writer,
        &Cell::Divider(DividerCell {
            id: "c2".to_owned(),
            read_only: None,
        }),
    );
    assert_eq!(writer.to_string(), "c.divider(),\n");
}

#[test]
fn print_cell_handles_unicode() {
    let mut writer = CodeWriter::new();
    let cell = Cell::Text(TextCell {
        content: "👀 I'm a text cell with unicode 🦀".to_owned(),
        ..Default::default()
    });
    print_cell(&mut writer, &cell);
    assert_eq!(
        writer.to_string(),
        "c.text(\"👀 I'm a text cell with unicode 🦀\"),\n"
    );
}

#[test]
fn print_cell_handles_formatted_unicode() {
    let mut writer = CodeWriter::new();
    let cell = Cell::Text(TextCell {
        content: "👀".to_owned(),
        formatting: vec![
            AnnotationWithOffset::new(0, Annotation::StartHighlight),
            AnnotationWithOffset::new(1, Annotation::EndHighlight),
        ],
        ..Default::default()
    });
    print_cell(&mut writer, &cell);
    assert_eq!(writer.to_string(), "c.text([fmt.highlight(['👀'])]),\n");
}
