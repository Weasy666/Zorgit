use std::fmt;
use std::str::FromStr;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use url::Url as ServoUrl;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};


#[derive(Clone)]
pub struct Url(ServoUrl);

impl Url {
}

impl Deref for Url {
    type Target = ServoUrl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ServoUrl> for Url {
    fn from(url: ServoUrl) -> Self {
        Url(url)
    }
}

impl<'a> FromFormValue<'a> for Url {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Url, &'a RawStr> {
        form_value.parse::<Url>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Url {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Url, Self::Error> {
        param.parse::<Url>()
            .map_err(|_| param)
    }
}


impl FromStr for Url {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Url, Self::Err> {
        ServoUrl::parse(s)
            .map(|url| Url(url))
    }
}

/// Display the serialization of this URL.
impl fmt::Display for Url {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Debug the serialization of this URL.
impl fmt::Debug for Url {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UrlVisitor;

        impl<'de> Visitor<'de> for UrlVisitor {
            type Value = Url;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("the following three parts of an url 'scheme', 'host' and 'port'")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut url_map: std::collections::HashMap<String,String> = std::collections::HashMap::with_capacity(access.size_hint().unwrap_or(0));

                while let Some((key, value)) = access.next_entry()? {
                    url_map.insert(key, value);
                }

                let host = url_map.get("host")
                    .map(|h| { if &h.to_lowercase() == "localhost" { "127.0.0.1" } else { h } })
                    .unwrap_or(&"127.0.0.1");

                let url = format!("{}://{}:{}", url_map.get("scheme").unwrap(), host, url_map.get("port").unwrap());
                let url = url.parse::<Url>().unwrap();

                Ok(url)
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E>
            {
                s.parse::<Url>().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_map(UrlVisitor)
    }
}