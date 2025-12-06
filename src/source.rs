pub struct SourceFile {
    pub name: String,
    pub text: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        let text = text.into();
        let mut line_starts = vec![0usize];
        for (i, ch) in text.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            name: name.into(),
            text,
            line_starts,
        }
    }

    pub fn get_line(&self, line: usize) -> &str {
        if line == 0 || line > self.line_starts.len() {
            return "";
        }

        let start = self.line_starts[line - 1];
        let end = if line < self.line_starts.len() {
            self.line_starts[line] - 1
        } else {
            self.text.len()
        };

        &self.text[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file() {
        let sf = SourceFile::new("empty", "");
        assert_eq!(sf.get_line(1), "");
        assert_eq!(sf.get_line(0), "");
    }

    #[test]
    fn test_single_line() {
        let sf = SourceFile::new("single", "hello world");
        assert_eq!(sf.get_line(1), "hello world");
        assert_eq!(sf.get_line(2), "");
    }

    #[test]
    fn test_multiple_lines() {
        let text = "line1\nline2\nline3";
        let sf = SourceFile::new("multi", text);
        assert_eq!(sf.get_line(1), "line1");
        assert_eq!(sf.get_line(2), "line2");
        assert_eq!(sf.get_line(3), "line3");
        assert_eq!(sf.get_line(4), "");
    }

    #[test]
    fn test_trailing_newline() {
        let text = "first\nsecond\n";
        let sf = SourceFile::new("trailing", text);
        assert_eq!(sf.get_line(1), "first");
        assert_eq!(sf.get_line(2), "second");
        assert_eq!(sf.get_line(3), "");
    }

    #[test]
    fn test_unicode() {
        let text = "αβγ\nδεζ\n";
        let sf = SourceFile::new("unicode", text);
        assert_eq!(sf.get_line(1), "αβγ");
        assert_eq!(sf.get_line(2), "δεζ");
        assert_eq!(sf.get_line(3), "");
    }
}
