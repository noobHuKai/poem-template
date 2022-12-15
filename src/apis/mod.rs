use crate::model::{JsonResponse, Response};
use poem::{handler, web::Path};
#[handler]
pub fn hello(Path(_name): Path<String>) -> JsonResponse<String> {
    Response::ok_with_data("data".to_string())
}
