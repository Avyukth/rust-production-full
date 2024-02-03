use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;

pub async fn mw_reponse_map(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
	res: Response,
) -> Response {


}
