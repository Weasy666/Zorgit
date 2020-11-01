use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use url::Url as ServoUrl;


#[derive(Clone)]
pub struct Url(ServoUrl);


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
    type Error = Box<dyn std::error::Error>;

    fn from_form_value(form_value: &'a RawStr) -> Result<Url, Self::Error> {
        Ok(form_value.parse::<Url>()?)
    }
}

impl<'a> FromParam<'a> for Url {
    type Error = Box<dyn std::error::Error>;

    fn from_param(param: &'a RawStr) -> Result<Url, Self::Error> {
        Ok(param.parse::<Url>()?)
    }
}


impl FromStr for Url {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Url, Self::Err> {
        Ok(ServoUrl::parse(s)
            .map(|url| Url(url))?)
    }
}

/// Display the serialization of this URL.
impl std::fmt::Display for Url {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Debug the serialization of this URL.
impl std::fmt::Debug for Url {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("the following three parts of an url 'scheme', 'host' and 'port'")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut url_map: HashMap<String,String> = HashMap::with_capacity(access.size_hint().unwrap_or(0));

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
