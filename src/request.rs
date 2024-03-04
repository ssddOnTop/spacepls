use anyhow::Context;
use anyhow::Result;
use bytes::Bytes;
use derive_setters::Setters;
use http_body_util::{BodyExt, Full};
use std::str::FromStr;

#[derive(Clone, Debug, Setters)]
pub struct Request<Body: Default + Clone> {
    pub url: url::Url,
    pub method: hyper::Method,
    pub headers: hyper::header::HeaderMap,
    pub body: Body,
}

impl<T: Default + Clone> Default for Request<T> {
    fn default() -> Self {
        Self {
            url: url::Url::parse("http://localhost").unwrap(),
            method: Default::default(),
            headers: Default::default(),
            body: T::default(),
        }
    }
}

impl Request<Bytes> {
    pub fn from_reqwest(req: reqwest::Request) -> Result<Self> {
        Ok(Self {
            url: req.url().clone(),
            method: to_hyper_method(req.method())?,
            headers: to_hyper_header(req.headers())?,
            body: req
                .body()
                .context("No body found in request.")?
                .as_bytes()
                .map(|v| Bytes::from(v.to_vec()))
                .context("Unable to convert body to bytes.")?,
        })
    }
    pub async fn from_hyper(req: hyper::Request<Full<Bytes>>) -> Result<Self> {
        let url = req.uri().to_string().parse()?;
        let method = req.method().clone();
        let headers = req.headers().clone();

        let body = if method == hyper::Method::POST {
            req
                .into_body()
                .frame()
                .await
                .context("unable to extract frame")??
                .into_data()
                .map_err(|e| anyhow::anyhow!("{:?}", e))?
        }else {
            Bytes::new()
        };

        Ok(Self {
            url,
            method,
            headers,
            body,
        })
    }
}

fn to_hyper_method(method: &reqwest::Method) -> Result<hyper::Method> {
    Ok(hyper::Method::from_str(method.as_str())?)
}

fn to_hyper_header(reqw_map: &reqwest::header::HeaderMap) -> Result<hyper::header::HeaderMap> {
    let mut header_map = hyper::header::HeaderMap::new();
    for (k, v) in reqw_map {
        header_map.insert(
            hyper::header::HeaderName::from_str(k.as_str())?,
            hyper::header::HeaderValue::from_bytes(v.as_bytes())?,
        );
    }
    Ok(header_map)
}
