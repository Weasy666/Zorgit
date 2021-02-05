use pulldown_cmark::{CowStr, Event, Parser, Options, html, Tag, CodeBlockKind};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::{easy::HighlightLines, html::{append_highlighted_html_for_styled_line, IncludeBackground}, util::LinesWithEndings};
use crate::models::SourceLine;
use crate::vcs::DiffHunk;
use std::path::Path;

pub fn markdown<P: AsRef<Path>>(root: P, markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut block_converter = BlockConverter::new();
    let parser = Parser::new_ext(markdown_input, options)
        .map(|event| make_link_absolut(event, root.as_ref()))
        .map(|event| block_converter.convert(event));
    
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}

struct BlockConverter {
    code_lang: Option<String>,
}

impl BlockConverter {
    fn new() -> Self {
        BlockConverter {
            code_lang: None,
        }
    }

    fn convert<'e>(&mut self, event: Event<'e>) -> Event<'e> {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                if let CodeBlockKind::Fenced(lang) = kind {
                    self.code_lang = Some(lang.clone().into_string());
                }
                event
            }
            Event::End(Tag::CodeBlock(_)) => {
                self.code_lang = None;
                event
            }
            Event::Text(ref text) if self.code_lang.is_some() => {
                let highlight = highlight_string_by_lang(text, self.code_lang.as_ref().unwrap());
                let highlight = highlight.strip_suffix("</pre>\n").unwrap();
                let highlight = highlight.splitn(2, '\n').last().unwrap();
                Event::Html(CowStr::from(highlight.to_string()))
            }
            _ => event,
        }
    }
}

fn make_link_absolut<'a, P: AsRef<Path>>(event: Event<'a>, path: P) -> Event<'a> {
    match event {
        Event::Start(Tag::Image(link_type, src, title)) => {
            let new_src =  if src.starts_with("http") { src } else { CowStr::from(format!("{}/{}", path.as_ref().display(), src))};
            Event::Start(Tag::Image(link_type, new_src.into(), title))
        }
        _ => event,
    }
}

pub fn highlight_hunks_by_extension(hunks: &[DiffHunk], extension: Option<&str>) -> Vec<DiffHunk> {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = extension.map(|e| ss.find_syntax_by_extension(e))
                        .flatten()
                        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let theme = load_theme();
    hunks.into_iter()
        .map(|hunk| {
            let mut hunk = hunk.to_owned();
            hunk.lines = highlight_source_lines(&ss, syntax, &theme, &hunk.lines);
            hunk
        })
        .collect()
}

pub fn highlight_source_lines_by_extension(lines: &[SourceLine], extension: Option<&str>) -> Vec<SourceLine> {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = extension.map(|e| ss.find_syntax_by_extension(e))
                        .flatten()
                        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlight_source_lines(&ss, syntax, &load_theme(), lines)
}

pub fn highlight_string_by_lang(string: &str, lang: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss.find_syntax_by_name(lang)
                        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlight_string_for_html(string, &ss, syntax, &load_theme())  
}

pub fn highlight_string_by_extension(string: &str, extension: Option<&str>) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = extension.map(|e| ss.find_syntax_by_extension(e))
                        .flatten()
                        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlight_string_for_html(string, &ss, syntax, &load_theme())  
}

fn highlight_source_lines(syntax_set: &SyntaxSet, syntax: &SyntaxReference, theme: &Theme, lines: &[SourceLine]) -> Vec<SourceLine> {
    lines.into_iter()
        .map(|source_line| {
            match &source_line.content {
                Some(content) => {
                    let mut source_line = source_line.to_owned();
                    source_line.content = Some(highlight_string_for_html(&content, syntax_set, syntax, theme));
                    source_line
                },
                None => source_line.to_owned(),
            }
        })
        .collect()
}

fn highlight_string_for_html(s: &str, ss: &SyntaxSet, syntax: &SyntaxReference, theme: &Theme) -> String {
    let mut highlighter = HighlightLines::new(syntax, theme);

    let bg = theme.settings.background
                        .map(|b| IncludeBackground::IfDifferent(b))
                        .unwrap_or(IncludeBackground::No);

    let color = match bg {
        IncludeBackground::IfDifferent(c) => format!(" style=\"background-color:#{:02x}{:02x}{:02x};\"", c.r, c.g, c.b),
        _ => String::default(),
    };
    //TODO: <pre> is just added here because of the background color. Should be extracted to wrap an entire code block instead of every single line
    let mut output = format!("<pre{}>\n", color);

    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight(line, ss);
        append_highlighted_html_for_styled_line(&regions[..], bg, &mut output);
    }
    output.push_str("</pre>\n");
    output
}

fn load_theme() -> Theme {
    //TODO: allow a user to choose a theme
    let ts = ThemeSet::load_defaults();
    let mut theme = ts.themes["InspiredGitHub"].clone();
    theme.settings.background = None;
    theme
}