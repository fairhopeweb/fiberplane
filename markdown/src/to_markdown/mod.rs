use fiberplane::protocols::core::{Cell, HeadingType, ListItemCell, ListType, Notebook};
use fiberplane::protocols::formatting::{Annotation, AnnotationWithOffset, Formatting};
use pulldown_cmark::Event::{self, *};
use pulldown_cmark::{CodeBlockKind, CowStr, HeadingLevel, LinkType, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::cmp::Ordering;
use tracing::warn;
use url::Url;

#[cfg(test)]
mod tests;

/// Convert the notebook to Markdown
pub fn notebook_to_markdown(notebook: Notebook) -> String {
    NotebookConverter::new().convert_to_markdown(notebook)
}

/// Convert the notebook to Markdown
///
/// The base URL is used to convert Image File IDs to URLs.
pub fn notebook_to_markdown_with_base_url(notebook: Notebook, base_url: Url) -> String {
    let mut converter = NotebookConverter::new();
    converter.set_base_url(base_url);
    converter.convert_to_markdown(notebook)
}

/// Convert the cells to Markdown
pub fn cells_to_markdown(cells: impl IntoIterator<Item = Cell>) -> String {
    let mut converter = NotebookConverter::new();
    converter.convert_cells("notebook_id", cells);
    converter.into_markdown()
}

/// Convert the cells to Markdown
///
/// The base URL is used to convert Image File IDs to URLs.
pub fn cells_to_markdown_with_base_url(
    cells: impl IntoIterator<Item = Cell>,
    base_url: Url,
) -> String {
    let mut converter = NotebookConverter::new();
    converter.set_base_url(base_url);
    converter.convert_cells("notebook_id", cells);
    converter.into_markdown()
}

struct NotebookConverter<'a> {
    events: Vec<Event<'a>>,
    base_url: Url,
    /// This is a stack of list-related tags that will need to be closed
    list_tag_stack: Vec<(u8, Tag<'a>)>,
    list_level: u8,
}

impl<'a> NotebookConverter<'a> {
    fn new() -> Self {
        NotebookConverter {
            events: Vec::new(),
            base_url: Url::parse("https://fiberplane.com").unwrap(),
            list_tag_stack: Vec::new(),
            list_level: 0,
        }
    }

    fn set_base_url(&mut self, base_url: Url) {
        self.base_url = base_url;
    }

    fn convert_to_markdown(mut self, notebook: Notebook) -> String {
        self.convert_title(notebook.title);
        self.convert_cells(&notebook.id, notebook.cells);
        self.into_markdown()
    }

    fn convert_cells(&mut self, notebook_id: &str, cells: impl IntoIterator<Item = Cell>) {
        let mut cells = cells.into_iter().peekable();
        while let Some(cell) = cells.next() {
            match cell {
                Cell::Checkbox(cell) => {
                    self.events.push(Start(Tag::Item));
                    self.events.push(TaskListMarker(cell.checked));
                    self.convert_formatted_text(cell.content, cell.formatting);
                    self.events.push(End(Tag::Item));
                }
                Cell::Code(cell) => self.convert_code_block(cell.content),
                Cell::Divider(_) => self.events.push(Rule),
                Cell::Elasticsearch(cell) => self.convert_code_block(cell.content),
                Cell::Heading(cell) => {
                    let level = match cell.heading_type {
                        HeadingType::H1 => HeadingLevel::H2,
                        HeadingType::H2 => HeadingLevel::H3,
                        HeadingType::H3 => HeadingLevel::H4,
                    };
                    let tag = Tag::Heading(level, None, Vec::new());
                    self.events.push(Start(tag.clone()));
                    self.convert_formatted_text(cell.content, cell.formatting);
                    self.events.push(End(tag));
                }
                Cell::Image(cell) => {
                    let url = if let Some(url) = cell.url {
                        url
                    } else if let Some(file_id) = cell.file_id {
                        self.base_url
                            .join(&format!("/api/files/{}/{}", notebook_id, file_id))
                            .unwrap()
                            .to_string()
                    } else {
                        warn!("Ignoring image cell that has no URL or file ID");
                        continue;
                    };
                    let tag = Tag::Image(LinkType::Inline, url.into(), "".into());
                    self.events.push(Start(Tag::Paragraph));
                    self.events.push(Start(tag.clone()));
                    self.events.push(End(tag));
                    self.events.push(End(Tag::Paragraph));
                }
                Cell::ListItem(cell) => {
                    self.start_new_list(&cell);
                    self.add_list_item(cell);

                    // Now, handle all of the other items in the list
                    // Note: we only support lists where all of the list item cells are adjacent to
                    // one another (there cannot be any other type of cell mixed into the list)
                    while let Some(Cell::ListItem(_)) = cells.peek() {
                        if let Some(Cell::ListItem(cell)) = cells.next() {
                            let cell_level = cell.level.unwrap_or_default();

                            // Depending on the cell's level, determine whether to start a nested list
                            // or close previously started items
                            match cell_level.cmp(&self.list_level) {
                                Ordering::Greater => self.start_new_list(&cell),
                                Ordering::Less => self.end_lists_to_level(cell_level),
                                Ordering::Equal => {
                                    // Close the previous item (because this is another item at the same level)
                                    self.events.push(End(Tag::Item));
                                }
                            }

                            self.add_list_item(cell);
                        }
                    }

                    self.end_all_lists();
                }
                Cell::Loki(cell) => self.convert_code_block(cell.content),
                Cell::Prometheus(cell) => self.convert_code_block(cell.content),
                Cell::Text(cell) => self.convert_formatted_text(cell.content, cell.formatting),
                Cell::Graph(_) => warn!("Ignoring Graph cell because they are not yet supported"),
                Cell::Log(_) => warn!("Ignoring Log cell because they are not yet supported"),
                Cell::Table(_) => warn!("Ignoring Table cell because they are not yet supported"),
            }
        }
    }

    fn into_markdown(self) -> String {
        let mut markdown = String::new();
        cmark(self.events.into_iter(), &mut markdown).unwrap();
        markdown
    }

    fn convert_title(&mut self, title: impl Into<CowStr<'a>>) {
        let tag = Tag::Heading(HeadingLevel::H1, None, Vec::new());
        self.events.push(Start(tag.clone()));
        self.text(title);
        self.events.push(End(tag));
    }

    fn convert_formatted_text(&mut self, content: String, formatting: Option<Formatting>) {
        let mut current_offset: usize = 0;
        let mut content = content.chars().peekable();
        let mut tags_to_close = Vec::new();

        let mut formatting = formatting.unwrap_or_default();
        formatting.sort_by_key(|a| a.offset);

        let mut formatting = formatting.into_iter().peekable();
        while let (Some(AnnotationWithOffset { offset, annotation }), next, content) =
            (formatting.next(), formatting.peek(), &mut content)
        {
            let offset = offset as usize;
            if offset > current_offset {
                self.text(content.take(offset - current_offset).collect::<String>());
                current_offset = offset;
            }

            // Ignore annotations that start after the end of the content
            // (Note that any unclosed annotations will be closed when
            // we clear out the stack after this loop)
            if content.peek().is_none() {
                break;
            }

            match annotation {
                // Simple formatting annotations
                Annotation::StartBold
                | Annotation::StartItalics
                | Annotation::StartStrikethrough
                | Annotation::StartUnderline
                | Annotation::StartLink { .. } => {
                    let tag = match annotation {
                        Annotation::StartBold => Tag::Strong,
                        Annotation::StartItalics => Tag::Emphasis,
                        Annotation::StartUnderline => Tag::Emphasis,
                        Annotation::StartStrikethrough => Tag::Strikethrough,
                        Annotation::StartLink { url } => {
                            Tag::Link(LinkType::Inline, url.into(), "".into())
                        }
                        _ => unreachable!(),
                    };
                    self.events.push(Start(tag.clone()));
                    tags_to_close.push(tag);
                }
                Annotation::EndBold
                | Annotation::EndItalics
                | Annotation::EndUnderline
                | Annotation::EndStrikethrough => {
                    let tag = match annotation {
                        Annotation::EndBold => Tag::Strong,
                        Annotation::EndItalics => Tag::Emphasis,
                        Annotation::EndUnderline => Tag::Emphasis,
                        Annotation::EndStrikethrough => Tag::Strikethrough,
                        _ => unreachable!(),
                    };

                    // Find the most recently added tag of the same type,
                    // remove it from the tags to close, and close it
                    if let Some(last_index) = tags_to_close.iter().rposition(|t| t == &tag) {
                        let event = End(tags_to_close.remove(last_index));
                        self.events.push(event);
                    } else {
                        warn!(
                            "Found end annotation for annotation that was not started: {:?}",
                            annotation
                        );
                    }
                }
                Annotation::EndLink => {
                    let last_link_tag = tags_to_close
                        .iter()
                        .rposition(|t| matches!(t, Tag::Link(..)));
                    if let Some(last_index) = last_link_tag {
                        let event = End(tags_to_close.remove(last_index));
                        self.events.push(event);
                    } else {
                        warn!(
                            "Found end annotation for annotation that was not started: {:?}",
                            annotation
                        );
                    }
                }
                // Mentions are turned into bold text
                Annotation::Mention(mention) => {
                    self.events.push(Start(Tag::Strong));
                    // We take an extra character for the @ symbol (present in the content but not the mention.name)
                    let mention_length = mention.name.chars().count() + 1;
                    current_offset += mention_length;
                    self.text(content.take(mention_length).collect::<String>());
                    self.events.push(End(Tag::Strong));
                }
                // Code is unusual because we need to include the content in the event
                // so we require that the next annotation be the end of the code block
                Annotation::StartCode => {
                    let code: String = match next {
                        Some(annotation) if annotation.annotation == Annotation::EndCode => {
                            let offset = annotation.offset as usize;
                            let code = content.take(offset - current_offset).collect();
                            current_offset = offset;
                            code
                        }
                        // An unclosed code annotation means all of the rest of the content is code
                        None => content.collect(),
                        _ => {
                            warn!("Formatting inside inline code is not supported, ignoring inline code");
                            continue;
                        }
                    };
                    self.events.push(Code(code.into()));
                }
                // Highlights are unusual because we turn them into inline HTML
                // so we require that the next annotation be the end of the highlight
                Annotation::StartHighlight => {
                    let text: String = match next {
                        Some(annotation) if annotation.annotation == Annotation::EndHighlight => {
                            let offset = annotation.offset as usize;
                            let text = content.take(offset - current_offset).collect();
                            current_offset = offset;
                            text
                        }
                        // An unclosed highlight means the rest of the content is highlighted
                        None => content.collect(),
                        _ => {
                            warn!(
                                "Formatting inside highlights is not supported, ignoring highlight"
                            );
                            continue;
                        }
                    };

                    self.events
                        .push(Html(format!("<mark>{}</mark>", text).into()));
                }
                // These are handled in the start annotation
                Annotation::EndCode | Annotation::EndHighlight => {}
            }
        }

        // Handle any remaining text
        let leftover_content = content.collect::<String>();
        if !leftover_content.is_empty() {
            self.text(leftover_content);
        }

        // Close any tags that were left open
        self.events.extend(tags_to_close.into_iter().rev().map(End));
    }

    fn convert_code_block(&mut self, content: String) {
        let tag = Tag::CodeBlock(CodeBlockKind::Fenced("".into()));
        self.events.push(Start(tag.clone()));
        self.events.push(Text(content.into()));
        self.events.push(End(tag.clone()));
    }

    fn text(&mut self, text: impl Into<CowStr<'a>>) {
        self.events.push(Text(text.into()));
    }

    fn start_new_list(&mut self, cell: &ListItemCell) {
        let start_tag = match cell.list_type {
            ListType::Ordered => Tag::List(Some(cell.start_number.unwrap_or(1) as u64)),
            ListType::Unordered => Tag::List(None),
        };
        let level = cell.level.unwrap_or_default();

        self.list_level = level;
        self.list_tag_stack.push((level, start_tag.clone()));

        self.events.push(Start(start_tag));
    }

    fn add_list_item(&mut self, cell: ListItemCell) {
        let level = cell.level.unwrap_or_default();

        self.list_tag_stack.push((level, Tag::Item));

        self.events.push(Start(Tag::Item));
        self.convert_formatted_text(cell.content, cell.formatting);
    }

    fn end_lists_to_level(&mut self, level: u8) {
        while let Some((tag_level, _)) = self.list_tag_stack.last() {
            if *tag_level == level {
                break;
            }

            let tag = self.list_tag_stack.pop().unwrap().1;
            self.events.push(End(tag));
        }
    }

    fn end_all_lists(&mut self) {
        while let Some((_, tag)) = self.list_tag_stack.pop() {
            self.events.push(End(tag));
        }
    }
}

impl<'a> Default for NotebookConverter<'a> {
    fn default() -> Self {
        Self::new()
    }
}
