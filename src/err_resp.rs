use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

pub fn err_resp(body: String, status: u16) -> anyhow::Result<Response<Full<Bytes>>> {
    Ok(Response::builder()
        .status(hyper::StatusCode::from_u16(status)?)
        .body(Full::new(Bytes::from(body)))?)
}
