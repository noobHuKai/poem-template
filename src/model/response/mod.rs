use poem::web::Json;
use serde::Serialize;

pub type JsonResponse<T> = Json<Response<T>>;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

const RESPONSE_STATUS_OK: i32 = 200;
const RESPONSE_STATUS_FAIL: i32 = 500;

impl<T> Response<T> {
    pub fn ok_with_data(data: T) -> JsonResponse<T> {
        Json(Self {
            code: RESPONSE_STATUS_OK,
            message: "".to_string(),
            data: Some(data),
        })
    }

    pub fn fail_with_msg(msg: String) -> JsonResponse<T> {
        Json(Self {
            code: RESPONSE_STATUS_FAIL,
            message: msg,
            data: None,
        })
    }
}
