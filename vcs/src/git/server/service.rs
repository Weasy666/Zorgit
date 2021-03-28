use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromFormField, ValueField, prelude::ErrorKind},
    http::{ContentType, Header},
};


pub enum Service {
    UploadPack,
    ReceivePack,
    Advertisement(String)
}

impl<'h> Service {
    pub fn as_str(&self) -> String {
        match self {
            Self::UploadPack => "git-upload-pack".to_string(),
            Self::ReceivePack => "git-receive-pack".to_string(),
            Self::Advertisement(service) => format!("git-{}", service),
        }
    }

    pub fn as_git_cmd(&self) -> String {
        match self {
            Self::UploadPack => "upload-pack".to_string(),
            Self::ReceivePack => "receive-pack".to_string(),
            Self::Advertisement(service) => format!("{}", service),
        }
    }
}

impl<'h> Into<Header<'h>> for Service {
    fn into(self) -> Header<'h> {
        match self {
            Self::UploadPack => ContentType::new("application", "x-git-upload-pack-result").into(),
            Self::ReceivePack => ContentType::new("application", "x-git-receive-pack-result").into(),
            Self::Advertisement(service) => ContentType::new("application", format!("x-git-{}-advertisement", service)).into(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Service {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value {
            "git-upload-pack" => Ok(Self::UploadPack),
            "git-receive-pack" => Ok(Self::ReceivePack),
            _ if field.value.starts_with("git-") => Ok(Self::Advertisement(field.value.to_string())),
            _ => Err(form::Error::from(ErrorKind::Unknown))?,
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("git_service")
            .unwrap_or(20.bytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);

        // Try to parse the name as UTF-8 or return an error if it fails.
        let service = std::str::from_utf8(bytes)?;
        match service {
            "git-upload-pack" => Ok(Self::UploadPack),
            "git-receive-pack" => Ok(Self::ReceivePack),
            _ if service.starts_with("git-") => Ok(Self::Advertisement(service.to_string())),
            _ => Err(form::Error::from(ErrorKind::Unknown))?,
        }
    }
}
