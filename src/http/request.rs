use http_body_util::BodyExt;
use hyper::{HeaderMap, Method, Uri};

#[derive(derive_getters::Getters)]
pub struct Request<Body> {
    method: Method,
    header: HeaderMap,
    uri: Uri,
    body: Body,
}

impl Request<hyper::body::Incoming> {
    pub fn from_hyper_request(req: hyper::Request<hyper::body::Incoming>) -> Self {
        let (parts, body) = req.into_parts();

        let header = parts.headers;
        let method = parts.method;
        let uri = parts.uri;
        
        Self {
            method,
            header,
            uri,
            body,
        }
    }
    pub async fn into_bytes(self) -> anyhow::Result<Request<bytes::Bytes>> {
        let body = self.body.collect().await?.to_bytes();
        Ok(Request {
            method: self.method,
            header: self.header,
            uri: self.uri,
            body,
        })
    }
}
