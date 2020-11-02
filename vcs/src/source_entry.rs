use std::path::PathBuf;
use crate::Commit;
use crate::SourceLine;

#[derive(Debug, Clone)]
pub struct SourceEntry {
    pub id: String,
    pub name: String,
    pub size: usize,
    /// Path to this entry, i.e. "src/" is the root of "src/main.rs".
    pub root: PathBuf,
    pub is_file: bool,
    pub extension: Option<String>,
    pub last_commit: Option<Commit>,
    pub content: Option<Vec<SourceLine>>,
}

impl std::fmt::Display for SourceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.name, self.id)
    }
}

impl SourceEntry {
    pub fn has_content(&self) -> bool {
        self.content.is_some() && !self.content.as_ref().unwrap().is_empty()
    }

    pub fn is_binary(&self) -> bool {
        self.has_content() && match self.content.as_ref().unwrap().first().unwrap() { SourceLine::Binary(_) => true, _ => false }
    }

    pub fn is_markdown(&self) -> bool {
        self.has_content() && self.extension.as_ref().unwrap_or(&String::default()) == &"md".to_string()
    }

    pub fn add_line(&mut self, line: SourceLine) {
        let mut content = Vec::new();
        if self.content.is_some() {
            content = self.content.as_ref().unwrap().to_vec();
        }
        content.push(line);
        self.content = Some(content);
    }

    pub fn relative_url(&self) -> String { // I would have liked to user Url here, but its not possible to create a pure relative Url. It wants a base.
        self.root.join(&self.name).to_str().unwrap().to_string()
    }

    pub fn lines(&self) -> Vec<SourceLine> {
        if self.content.is_some() {
            self.content.as_ref().unwrap().to_vec()
        }
        else {
            Vec::new()
        }
    }

    pub fn content_as_string(&self) -> String {
        let mut content = String::default();
        if self.content.is_some() {
            content = self.content.as_ref().unwrap().iter()
                .map(|l| l.content.as_ref().unwrap_or(&"".to_string()).to_string())
                .collect::<Vec<String>>()
                .join("\n")
        }
        content
    }
}
