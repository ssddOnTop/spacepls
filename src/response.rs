use anyhow::{Context, Result};
use bytes::Bytes;
use derive_setters::Setters;
use http_body_util::Full;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Setters)]
pub struct Response<Body: Default + Clone> {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: Body,
}

impl Response<Bytes> {
    pub async fn from_reqwest(resp: reqwest::Response) -> Result<Self> {
        let status = resp.status();
        let headers = resp.headers().to_owned();
        let body = resp.bytes().await?;
        Ok(Response {
            status,
            headers,
            body,
        })
    }
    pub fn empty() -> Self {
        Response {
            status: reqwest::StatusCode::OK,
            headers: reqwest::header::HeaderMap::default(),
            body: Bytes::new(),
        }
    }

    pub fn to_json(self) -> Result<Response<serde_json::Value>> {
        let mut resp = Response::default();
        let body = serde_json::from_slice::<serde_json::Value>(&self.body)?;
        resp.body = body;
        resp.status = self.status;
        resp.headers = self.headers;
        Ok(resp)
    }

    pub fn to_resp_string(self) -> Result<Response<String>> {
        Ok(Response::<String> {
            body: String::from_utf8(self.body.to_vec())?,
            status: self.status,
            headers: self.headers,
        })
    }
    pub fn into_hyper(self) -> Result<hyper::Response<Full<Bytes>>> {
        let mut builder =
            hyper::Response::builder().status(hyper::StatusCode::from_u16(self.status.as_u16())?);
        for (key, value) in self.headers {
            builder = builder.header(
                hyper::header::HeaderName::from_str(
                    key.context("Invalid header key")?
                        .as_str()
                        .to_string()
                        .as_str(),
                )?,
                hyper::header::HeaderValue::from_str(value.to_str()?)?,
            );
        }
        Ok(builder.body(Full::new(self.body))?)
    }
}
