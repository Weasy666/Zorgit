use std::fmt;
use std::str::FromStr;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};


#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn uni_lang_id(&self) -> &'static str {
        match self {
            Language::English => ENGLISH,
            Language::German => GERMAN,
        }
    }

    pub fn as_vec() -> Vec<Language> {
        LANGUAGES.to_vec()
    }
}

impl<'a> FromFormValue<'a> for Language {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Language, Self::Error> {
        form_value.parse::<Language>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Language {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Language, Self::Error> {
        param.parse::<Language>()
            .map_err(|_| param)
    }
}

impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ENGLISH => Ok(Language::English),
            GERMAN => Ok(Language::German),
            _ => Err(anyhow!("A language with the following Unicode Language Identifier does not exist: {}", s)),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uni_lang_id())
    }
}

pub const LANGUAGES: [Language;  2] = [Language::English, Language::German];
const ENGLISH: &str = "en-EN";
const GERMAN: &str = "de-DE";