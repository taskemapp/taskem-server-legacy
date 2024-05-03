use crate::domain::constants::MIDDLEWARE_AUTH_SESSION_KEY;
use crate::domain::repositories::session::RedisSessionRepository;
use derive_new::new;
use hyper::StatusCode;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::codegen::http::{Request, Response};
use tonic::transport::Body;
use tower::Layer;
use tower_service::Service;
use tracing::{debug, info};

#[derive(Clone, new)]
pub struct AuthMiddlewareLayer {
    redis_repository: Arc<dyn RedisSessionRepository>,
}

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            redis_repository: self.redis_repository.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    redis_repository: Arc<dyn RedisSessionRepository>,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        debug!("Check auth url for request: \n{:?}", req.uri());

        if req.uri().path().contains("Auth") {
            return Box::pin(async move {
                let response = inner.call(req).await?;
                Ok(response)
            });
        }

        let headers = req.headers();

        debug!("Find authorization header");

        if let Some(header_value) = headers.get(MIDDLEWARE_AUTH_SESSION_KEY) {
            let value = header_value
                .to_str()
                .expect("Can't convert header value to str");
            let session = format!("session_id:{}", value);

            if self.redis_repository.session_expand(&session).is_ok() {
                info!("Session expanded");
                return Box::pin(async move {
                    let response = inner.call(req).await?;
                    Ok(response)
                });
            }
        }

        debug!("Unauthorized");

        Box::pin(async move {
            let res = Response::builder()
                .status(StatusCode::UNAUTHORIZED.as_u16())
                .body(BoxBody::default())
                .unwrap();
            Ok(res)
        })
    }
}

impl<S> Service<axum::extract::Request> for AuthMiddleware<S>
where
    S: Service<axum::extract::Request, Response = axum::response::Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: axum::extract::Request) -> Self::Future {
        let headers = request.headers();

        debug!("Find authorization header");

        if let Some(header_value) = headers.get(MIDDLEWARE_AUTH_SESSION_KEY) {
            let value = header_value
                .to_str()
                .expect("Can't convert header value to str");
            let session = format!("session_id:{}", value);

            if self.redis_repository.session_expand(&session).is_ok() {
                info!("Session expanded");
                let future = self.inner.call(request);
                return Box::pin(async move {
                    let response: axum::response::Response = future.await?;
                    Ok(response)
                });
            }
        }

        debug!("Unauthorized");
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::empty())
                .unwrap())
        })
    }
}
