use std::io::{BufWriter, Write};

use async_trait::async_trait;

pub mod commands;
pub mod error;
pub mod regexes;

#[async_trait]
pub trait WebFetch {
    async fn fetch(&self, url: &str) -> Result<String, crate::error::Error>;
}

pub struct HttpReqwest;

#[async_trait]
impl WebFetch for HttpReqwest {
    async fn fetch(&self, url: &str) -> Result<String, crate::error::Error> {

        match reqwest::get(url).await {
            Ok(get_response) => match get_response.error_for_status() {
                Ok(resp) => match resp.text().await {
                    Ok(text) => Ok(text),
                    Err(err) => Err(error::Error::from(err)),
                },
                Err(err) => Err(error::Error::from(err)),
            },
            Err(err) => Err(error::Error::from(err)),
        }
    }
}

impl HttpReqwest {
    pub fn new() -> Self {
        return Self {};
    }
}

pub fn save_to_json(filename: &str, data: impl serde::Serialize) -> Result<(), crate::error::Error> {
    let mut buf_writer = BufWriter::new(std::fs::File::create(filename)?);
    serde_json::to_writer_pretty(&mut buf_writer, &data)?;
    buf_writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod fakers {
    use super::error::Error;
    use super::WebFetch;
    use super::async_trait;

    #[derive(Default)]
    pub struct FakeHttpReqwest {
        success_response: String,
        error_response: Option<Error>,
    }

    impl FakeHttpReqwest {
        pub fn set_success_response(mut self, response: String) -> Self {
            self.success_response = response;

            return self;
        }

        pub fn set_error_response(mut self, error: Error) -> Self {
            self.error_response = Some(error);

            return self;
        }
    }

    #[async_trait]
    impl WebFetch for FakeHttpReqwest {
        async fn fetch(&self, _: &str) -> Result<String, Error> {
            if let Some(err) = &self.error_response {
                return Err(err.clone());
            }

            return Ok(self.success_response.clone());
        }
    }
}

