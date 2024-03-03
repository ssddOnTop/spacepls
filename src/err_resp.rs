use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

pub fn err_resp(body: String) -> anyhow::Result<Response<Full<Bytes>>> {
    Ok(
        Response::builder()
            .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Full::new(Bytes::from(body)))?
    )
}