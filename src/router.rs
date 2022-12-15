use poem::{get, Route};

use crate::apis;

pub fn init_router() -> Route {
    Route::new().at("/hello/:name", get(apis::hello))
}
