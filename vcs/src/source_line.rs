use std::ops::{Deref, DerefMut};

//TODO: Write a macro that replaces SourceLineInner
#[derive(Debug, Clone)]
pub enum SourceLine {
	Plain(SourceLineInner),
	Add(SourceLineInner),
	Del(SourceLineInner),
	Binary(SourceLineInner),
	Hunk(SourceLineInner),
}

impl SourceLine {
    pub fn new_plain(old_num: Option<u32>, new_num: Option<u32>, marker: char, content: Option<&str>) -> SourceLine {
        SourceLine::Plain(SourceLineInner {
            old_num,
            new_num,
            marker,
            content: content.map(ToString::to_string),
        })
    }

    pub fn new_binary(old_num: Option<u32>, new_num: Option<u32>, marker: char, content: Option<&str>) -> SourceLine {
        SourceLine::Binary(SourceLineInner {
            old_num,
            new_num,
            marker,
            content: content.map(ToString::to_string),
        })
    }

    pub fn css_class(&self) -> &'_ str {
        match &self {
            SourceLine::Plain(_)  => "code plain",
            SourceLine::Add(_)    => "code add",
            SourceLine::Del(_)    => "code del",
            SourceLine::Binary(_) => "code binary",
            SourceLine::Hunk(_)   => "code hunk",
        }
    }
}

impl Deref for SourceLine {
    type Target = SourceLineInner;

    fn deref(&self) -> &Self::Target {
        match self {
            SourceLine::Plain(l)  => l,
            SourceLine::Add(l)    => l,
            SourceLine::Del(l)    => l,
            SourceLine::Binary(l) => l,
            SourceLine::Hunk(l)   => l,
        }
    }
}

impl DerefMut for SourceLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            SourceLine::Plain(l)  => l,
            SourceLine::Add(l)    => l,
            SourceLine::Del(l)    => l,
            SourceLine::Binary(l) => l,
            SourceLine::Hunk(l)   => l,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceLineInner {
    pub old_num:    Option<u32>,
	pub new_num:    Option<u32>,
    pub marker:     char,
	pub content:    Option<String>,
}

impl SourceLineInner {
    pub fn old_num(&self) -> String {
        self.old_num.map_or("".to_string(),|n| n.to_string())
    }

    pub fn new_num(&self) -> String {
        self.new_num.map_or("".to_string(), |n| n.to_string())
    }

    pub fn content_as_string(&self) -> String {
        self.content.as_ref().unwrap_or(&String::default()).to_string()
    }
}
