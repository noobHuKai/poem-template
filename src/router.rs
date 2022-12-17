use poem::{post, Route};

use crate::apis;

pub fn init_router() -> Route {
    Route::new().nest("/apis", api_router())
}
fn api_router() -> Route {
    Route::new().nest("/user", user_router())
}

fn user_router() -> Route {
    Route::new().at("/login", post(apis::user::login))
}
