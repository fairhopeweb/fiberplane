use std::fmt::Display;

/// Simple helper that maintains the current indentation level
/// and a buffer of strings
pub(crate) struct CodeWriter {
    buffer: Vec<String>,
    indent: u8,
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter {
            buffer: Vec::new(),
            indent: 0,
        }
    }

    /// Increase the indentation level
    pub fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }

    /// Decrease the indentation level
    pub fn dedent(&mut self) -> &mut Self {
        self.indent -= 1;
        self
    }

    /// Add a new line to the buffer at the current indentation level
    pub fn println(&mut self, line: impl Into<String>) -> &mut Self {
        self.buffer.push("  ".repeat(self.indent as usize));
        self.buffer.push(line.into());
        self.buffer.push("\n".to_string());
        self
    }

    /// Increase the indentation level,
    /// add a new line to the buffer,
    /// and then decrease the indentation level again
    pub fn println_indented(&mut self, line: impl Into<String>) -> &mut Self {
        self.indent();
        self.println(line);
        self.dedent();
        self
    }
}

impl Display for CodeWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.buffer {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[test]
fn handles_indentation() {
    let output = CodeWriter::new()
        .println("a")
        .indent()
        .println("b")
        .println_indented("c")
        .println("d")
        .dedent()
        .println("e")
        .to_string();
    assert_eq!(
        output,
        "a
  b
    c
  d
e
"
    );
}

#[test]
fn handles_multi_line_strings() {
    let output = CodeWriter::new()
        .println("")
        .println("a")
        .indent()
        .indent()
        .println("b\nc")
        .to_string();
    assert_eq!(
        output,
        "
a
    b
c
"
    );
}
