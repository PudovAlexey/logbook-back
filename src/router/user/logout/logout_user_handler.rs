use http::Response;
use serde_json::json;

use crate::{
    common::jwt::remove_jwt_cookie,
    error::{into_response, AppResult, SuccessResponse},
};

pub async fn logout_user_handler() -> AppResult<SuccessResponse<String>> {
    let res = Response::new(json!({"status": "success"}).to_string());

    remove_jwt_cookie(res);

    Ok(into_response(String::from("success")))
}
