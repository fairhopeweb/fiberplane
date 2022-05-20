use super::*;
use fiberplane::protocols::core::*;
use fiberplane::protocols::formatting::{Annotation, AnnotationWithOffset, Mention};

#[test]
fn title() {
    let mut converter = NotebookConverter::new();
    converter.convert_title("Some title");
    let markdown = converter.into_markdown();
    assert_eq!(markdown, "# Some title");
}

#[test]
fn decrements_headings() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook-id",
        [
            Cell::Heading(HeadingCell {
                content: "H1".to_string(),
                heading_type: HeadingType::H1,
                ..Default::default()
            }),
            Cell::Heading(HeadingCell {
                content: "H2".to_string(),
                heading_type: HeadingType::H2,
                ..Default::default()
            }),
        ],
    );
    assert_eq!(converter.into_markdown(), "## H1\n\n### H2");
}

#[test]
fn plain_text() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text("Some text".to_string(), None);
    assert_eq!(converter.into_markdown(), "Some text");
}

#[test]
fn formatted_text() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some bold, italics, strikethrough".to_string(),
        Some(vec![
            AnnotationWithOffset::new(5, Annotation::StartBold),
            AnnotationWithOffset::new(9, Annotation::EndBold),
            AnnotationWithOffset::new(11, Annotation::StartItalics),
            AnnotationWithOffset::new(18, Annotation::EndItalics),
            AnnotationWithOffset::new(20, Annotation::StartStrikethrough),
            AnnotationWithOffset::new(33, Annotation::EndStrikethrough),
        ]),
    );
    assert_eq!(
        converter.into_markdown(),
        "Some **bold**, *italics*, ~~strikethrough~~"
    );
}

#[test]
fn text_cells() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook-id",
        [
            Cell::Text(TextCell {
                content: "Some text".to_string(),
                ..Default::default()
            }),
            Cell::Text(TextCell {
                content: "Some more text".to_string(),
                ..Default::default()
            }),
        ],
    );
    assert_eq!(converter.into_markdown(), "Some text\n\nSome more text");
}

#[test]
fn mentions() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some @mention".to_string(),
        Some(vec![AnnotationWithOffset::new(
            5,
            Annotation::Mention(Mention {
                name: "mention".to_string(),
                user_id: "user_id".to_string(),
            }),
        )]),
    );
    assert_eq!(converter.into_markdown(), "Some **@mention**");
}

#[test]
fn links() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some link here".to_string(),
        Some(vec![
            AnnotationWithOffset::new(
                5,
                Annotation::StartLink {
                    url: "https://www.rust-lang.org".to_string(),
                },
            ),
            AnnotationWithOffset::new(9, Annotation::EndLink),
        ]),
    );
    assert_eq!(
        converter.into_markdown(),
        "Some [link](https://www.rust-lang.org) here"
    );
}

#[test]
fn images() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        vec![
            Cell::Image(ImageCell {
                url: Some("http://example.com/image.png".to_string()),
                ..Default::default()
            }),
            Cell::Image(ImageCell {
                file_id: Some("file_id".to_string()),
                ..Default::default()
            }),
        ],
    );
    let markdown = converter.into_markdown();
    assert_eq!(markdown, "![](http://example.com/image.png)\n\n![](https://fiberplane.com/api/files/notebook_id/file_id)");
}

#[test]
fn inline_code() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some code here".to_string(),
        Some(vec![
            AnnotationWithOffset::new(5, Annotation::StartCode),
            AnnotationWithOffset::new(9, Annotation::EndCode),
        ]),
    );
    assert_eq!(converter.into_markdown(), "Some `code` here");
}

#[test]
fn code_blocks() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook-id",
        [
            Cell::Code(CodeCell {
                content: "Some code".to_string(),
                ..Default::default()
            }),
            Cell::Code(CodeCell {
                content: "Some more code\non multiple lines".to_string(),
                ..Default::default()
            }),
        ],
    );
    assert_eq!(
        converter.into_markdown(),
        "
```
Some code
```

```
Some more code
on multiple lines
```"
    );
}

#[test]
fn unclosed_formatting_annotation() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some bold".to_string(),
        Some(vec![AnnotationWithOffset::new(5, Annotation::StartBold)]),
    );
    assert_eq!(converter.into_markdown(), "Some **bold**");
}

#[test]
fn unclosed_code() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some code".to_string(),
        Some(vec![AnnotationWithOffset::new(5, Annotation::StartCode)]),
    );
    assert_eq!(converter.into_markdown(), "Some `code`");
}

#[test]
fn ignore_start_formatting_annotation_at_content_end() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some text".to_string(),
        Some(vec![
            AnnotationWithOffset::new(9, Annotation::StartBold),
            AnnotationWithOffset::new(10, Annotation::EndBold),
        ]),
    );
    assert_eq!(converter.into_markdown(), "Some text");
}

#[test]
fn mixed_formatting() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "A link with code and bold here".to_string(),
        Some(vec![
            AnnotationWithOffset::new(
                2,
                Annotation::StartLink {
                    url: "http://example.com".to_string(),
                },
            ),
            AnnotationWithOffset::new(12, Annotation::StartCode),
            AnnotationWithOffset::new(16, Annotation::EndCode),
            AnnotationWithOffset::new(21, Annotation::StartBold),
            AnnotationWithOffset::new(25, Annotation::EndBold),
            AnnotationWithOffset::new(25, Annotation::EndLink),
        ]),
    );

    assert_eq!(
        converter.into_markdown(),
        "A [link with `code` and **bold**](http://example.com) here"
    );
}

#[test]
fn overlapping_formatting() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some overlapping formatting".to_string(),
        Some(vec![
            AnnotationWithOffset::new(0, Annotation::StartBold),
            AnnotationWithOffset::new(5, Annotation::StartItalics),
            AnnotationWithOffset::new(16, Annotation::EndBold),
            AnnotationWithOffset::new(27, Annotation::EndItalics),
        ]),
    );
    let markdown = converter.into_markdown();
    assert_eq!(markdown, "**Some *overlapping** formatting*");
}

#[test]
fn highlighting() {
    let mut converter = NotebookConverter::new();
    converter.convert_formatted_text(
        "Some highlighted text".to_string(),
        Some(vec![
            AnnotationWithOffset::new(5, Annotation::StartHighlight),
            AnnotationWithOffset::new(21, Annotation::EndHighlight),
        ]),
    );
    assert_eq!(
        converter.into_markdown(),
        "Some <mark>highlighted text</mark>"
    );
}

#[test]
fn ordered_lists_without_start_number() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        [
            Cell::ListItem(ListItemCell {
                content: "one".to_string(),
                list_type: ListType::Ordered,
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "two".to_string(),
                list_type: ListType::Ordered,
                ..Default::default()
            }),
        ],
    );
    assert_eq!(converter.into_markdown(), "1. one\n1. two");
}

#[test]
fn ordered_lists_with_start_number() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        [
            Cell::ListItem(ListItemCell {
                content: "two".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(2),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "three".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(3),
                ..Default::default()
            }),
        ],
    );
    assert_eq!(
        converter.into_markdown(),
        "\
2. two
2. three"
    );
}
#[test]
fn nested_ordered_lists() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        [
            Cell::ListItem(ListItemCell {
                content: "one".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(1),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "one-one".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(1),
                level: Some(1),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "one-two".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(2),
                level: Some(1),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "two".to_string(),
                list_type: ListType::Ordered,
                start_number: Some(2),
                ..Default::default()
            }),
        ],
    );
    assert_eq!(
        converter.into_markdown(),
        "\
1. one
   1. one-one
   1. one-two
1. two"
    );
}

#[test]
fn nested_unordered_lists() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        [
            Cell::ListItem(ListItemCell {
                content: "one".to_string(),
                list_type: ListType::Unordered,
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "one-one".to_string(),
                list_type: ListType::Unordered,
                level: Some(1),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "one-two".to_string(),
                list_type: ListType::Unordered,
                level: Some(1),
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "two".to_string(),
                list_type: ListType::Unordered,
                ..Default::default()
            }),
        ],
    );
    assert_eq!(
        converter.into_markdown(),
        "\
- one
  - one-one
  - one-two
- two"
    );
}

#[test]
fn checkboxes() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook-id",
        [
            Cell::Checkbox(CheckboxCell {
                content: "one".to_string(),
                checked: true,
                ..Default::default()
            }),
            Cell::Checkbox(CheckboxCell {
                content: "two".to_string(),
                checked: false,
                ..Default::default()
            }),
        ],
    );
    let markdown = converter.into_markdown();
    assert_eq!(markdown, "- [x] one\n- [ ] two\n");
}

#[test]
fn text_cells_after_lists() {
    let mut converter = NotebookConverter::new();
    converter.convert_cells(
        "notebook_id",
        [
            Cell::ListItem(ListItemCell {
                content: "one".to_string(),
                list_type: ListType::Ordered,
                ..Default::default()
            }),
            Cell::ListItem(ListItemCell {
                content: "two".to_string(),
                list_type: ListType::Ordered,
                ..Default::default()
            }),
            Cell::Text(TextCell {
                content: "three".to_string(),
                ..Default::default()
            }),
            Cell::Checkbox(CheckboxCell {
                content: "four".to_string(),
                ..Default::default()
            }),
            Cell::Checkbox(CheckboxCell {
                content: "five".to_string(),
                ..Default::default()
            }),
            Cell::Text(TextCell {
                content: "six".to_string(),
                ..Default::default()
            }),
        ],
    );
    assert_eq!(
        converter.into_markdown(),
        "\
1. one
1. two

three

- [ ] four
- [ ] five

six"
    );
}
