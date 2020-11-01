use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::str::FromStr;
use rocket::http::uri::{Formatter, FromUriParam, Path, Segments, SegmentError, UriDisplay, UriPart};
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};


pub struct DotFile(pub PathBuf);

impl<I: AsRef<str>> From<I> for DotFile {
    fn from(input: I) -> Self {
        DotFile(PathBuf::from(input.as_ref()))
    }
}

impl rocket::request::FromSegments<'_> for DotFile {
    type Error = SegmentError;
    fn from_segments(segments: Segments<'_>) -> Result<DotFile, SegmentError> {
        segments.into_path_buf(true).map(|p| DotFile(p))
    }
}

impl<P: UriPart> UriDisplay<P> for DotFile {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_, P>) -> std::fmt::Result {
        self.0.to_str().unwrap().fmt(f)
    }
}

impl<'a, P: AsRef<std::path::Path>> FromUriParam<Path, P> for DotFile {
    type Target = DotFile;

    fn from_uri_param(param: P) -> DotFile {
         DotFile(PathBuf::from(param.as_ref()))
    }
}

rocket::http::impl_from_uri_param_identity!([Path] DotFile);

impl<'a> FromParam<'a> for DotFile {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<DotFile, Self::Error> {
        let cow = param.percent_decode()
            .map_err(|_| param)?;
        cow.parse::<DotFile>()
            .map_err(|_| param)
    }
}

impl<'a> FromFormValue<'a> for DotFile {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<DotFile, Self::Error> {
        form_value.parse::<String>()
            .map(|path| DotFile(PathBuf::from(&path)))
            .map_err(|_| form_value)
    }
}

impl FromStr for DotFile {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<DotFile, Self::Err> {
        let dot_file = PathBuf::from_str(s)?;
        Ok(DotFile(dot_file))
    }
}

impl Deref for DotFile {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DotFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
