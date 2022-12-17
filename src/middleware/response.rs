use crate::model::Response as res;
use poem::{async_trait, web::Json, Endpoint, IntoResponse, Middleware, Request, Response, Result};
pub struct ResponseMiddleware;

impl<E: Endpoint> Middleware<E> for ResponseMiddleware {
    type Output = ResponseImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ResponseImpl(ep)
    }
}

pub struct ResponseImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for ResponseImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let res = self.0.call(req).await;

        match res {
            Ok(resp) => Ok(resp.into_response()),
            Err(err) => {
                let a: Json<res<String>> = res::<String>::fail_with_msg(err.to_string());
                Ok(a.into_response())
            }
        }
    }
}
