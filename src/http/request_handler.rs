use std::sync::Arc;
use anyhow::Result;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Response};
use crate::{AppContext, Request};

const SUPPORTED_METHOD: &[&str] = &["GET", "POST"];


pub async fn handle_req(req: Request<Bytes>, app_ctx: Arc<AppContext>) -> Result<Response<Full<Bytes>>> {
    match req.method {
        Method::POST => handle_post(req, app_ctx).await,
        Method::GET => handle_get(req, app_ctx).await,
        _ => not_found(),
    }
}

/// Post requests should return a json response
async fn handle_post(req: Request<Bytes>, app_ctx: Arc<AppContext>) -> Result<Response<Full<Bytes>>> {
    let path = req.url.path();
    match path {
        "/query" => {
            todo!()
        }
        "/home" => {
            todo!()
        }
        &_ => {
            todo!()
        }
    }
}

/// Get requests should return a html response
async fn handle_get(req: Request<Bytes>, app_ctx: Arc<AppContext>) -> Result<Response<Full<Bytes>>> {
    let path = req.url.path();
    match path {
        "/query" => {
            todo!()
        }
        "/" | "/home" => {
            todo!()
        }
        &_ => {
            todo!()
        }
    }
}

fn not_found() -> Result<Response<Full<Bytes>>> {
    todo!()
}