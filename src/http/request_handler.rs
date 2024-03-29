use crate::AppContext;
use anyhow::Result;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Response};
use lazy_static::lazy_static;
use std::sync::Arc;

pub async fn handle_request(
    req: hyper::Request<hyper::body::Incoming>,
    app_ctx: Arc<AppContext>,
) -> Result<Response<Full<Bytes>>> {
    match *req.method() {
        Method::POST => handle_post(req, app_ctx).await,
        Method::GET => handle_get(req, app_ctx).await,
        _ => not_found(),
    }
}

/// Post requests should return a json response
async fn handle_post(
    req: hyper::Request<hyper::body::Incoming>,
    _app_ctx: Arc<AppContext>,
) -> Result<Response<Full<Bytes>>> {
    let path = req.uri().path();
    match path {
        "/query" => {
            todo!()
        }
        "/home" => {
            todo!()
        }
        &_ => not_found(),
    }
}

/// Get requests should return a html response
async fn handle_get(
    req: hyper::Request<hyper::body::Incoming>,
    _app_ctx: Arc<AppContext>,
) -> Result<Response<Full<Bytes>>> {
    let path = req.uri().path();
    match path {
        "/query" => {
            todo!()
        }
        "/" | "/home" => {
            todo!()
        }
        &_ => not_found(),
    }
}

lazy_static! {
    static ref PAGE_404: String = {
        let html = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/html/404.html"));
        let routes = serde_json::json!({
            "available_routes": ["home", "query"],
            "home": "It contains Homepage",
            "query": "You can query files here",
        });
        let routes = routes.to_string();
        let html = html.replace("SOME_UNIQUE_STRING_TO_BE_REPLACED", routes.as_str());
        html
    };
}

fn not_found() -> Result<Response<Full<Bytes>>> {
    let response = Response::builder()
        .status(404)
        .header("Content-Type", "text/html")
        .body(Full::new(Bytes::from(PAGE_404.as_str())))?;
    Ok(response)
}

#[cfg(test)]
mod test {
    use crate::http::request_handler::PAGE_404;
    use anyhow::Result;

    #[tokio::test]
    async fn contains_all_routes() -> Result<()> {
        assert!(PAGE_404.as_str().contains("home"));
        assert!(PAGE_404.as_str().contains("query"));
        Ok(())
    }
}
