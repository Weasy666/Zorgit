use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromForm, FromFormField, ValueField}
};
use serde::{Serialize, de::{self, Deserialize, Deserializer, Visitor, MapAccess}};
use url::Url as ServoUrl;


#[derive(Clone, Serialize)]
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

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Url {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(field.value.parse::<Url>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("url")
            .unwrap_or(64.kibibytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        // Try to parse the name as UTF-8 or return an error if it fails.
        let url = std::str::from_utf8(bytes)?;
        Ok(url.parse::<Url>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
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
