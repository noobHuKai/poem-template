use crate::model::{request::LoginReq, JsonResponse, Response};
use crate::service::user::login_service;
use poem::{
    handler,
    web::{Data, Json},
};
use sqlx::PgPool;

#[handler]
pub async fn login(conn: Data<&PgPool>, Json(req): Json<LoginReq>) -> JsonResponse<String> {
    match login_service(conn.0, req.username, req.password).await {
        Ok(user) => {
            println!("{:?}", user);
        }
        Err(err) => return Response::fail_with_msg(err.to_string()),
    }
    Response::ok_with_data("data".to_string())
}
