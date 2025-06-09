use anyhow::Result;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight_line(&self, line: &str, file_path: &Path, _line_num: usize) -> Result<String> {
        // Get syntax definition from file extension with fallbacks for TypeScript
        let syntax = self.get_syntax_for_file(file_path);

        let theme = &self.theme_set.themes["base16-eighties.dark"];

        let mut highlighter = HighlightLines::new(syntax, theme);

        // Highlight the line
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set)?;

        // Convert to terminal escape codes
        Ok(as_24_bit_terminal_escaped(&ranges[..], false))
    }

    fn get_syntax_for_file(&self, file_path: &Path) -> &syntect::parsing::SyntaxReference {
        // First try the file extension
        if let Ok(Some(syntax)) = self.syntax_set.find_syntax_for_file(file_path) {
            return syntax;
        }

        // Handle special cases for TypeScript and JSX files
        if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "ts" | "tsx" => {
                    // Use JavaScript syntax for TypeScript files as fallback
                    if let Some(js_syntax) = self.syntax_set.find_syntax_by_extension("js") {
                        return js_syntax;
                    }
                }
                "jsx" => {
                    // Use JavaScript syntax for JSX files
                    if let Some(js_syntax) = self.syntax_set.find_syntax_by_extension("js") {
                        return js_syntax;
                    }
                }
                "mdx" => {
                    // Use Markdown syntax for MDX files
                    if let Some(md_syntax) = self.syntax_set.find_syntax_by_extension("md") {
                        return md_syntax;
                    }
                }
                _ => {}
            }
        }

        // Fallback to plain text
        self.syntax_set.find_syntax_plain_text()
    }
}
