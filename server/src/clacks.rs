//! GNU Terry Pratchett
//!
//! "A man is not dead while his name is still spoken."
//! - Going Postal, Terry Pratchett
//!
//! This middleware adds the X-Clacks-Overhead header to all responses
//! as a tribute to Sir Terry Pratchett.
//!
//! See: http://www.gnuterrypratchett.com/

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

/// Middleware to add X-Clacks-Overhead header to all responses
///
/// In Terry Pratchett's "Going Postal," the clacks system (a semaphore network)
/// had a special code "GNU" meaning:
/// - G: send the message on
/// - N: do not log the message
/// - U: turn the message around at the end of the line
///
/// This code was used to keep the names of deceased clacks operators alive
/// in the network forever. The X-Clacks-Overhead header does the same for
/// Terry Pratchett on the internet.
pub async fn add_clacks_header(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "X-Clacks-Overhead",
        "GNU Terry Pratchett".parse().unwrap(),
    );

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn test_handler() -> &'static str {
        "test"
    }

    #[tokio::test]
    async fn test_clacks_header_is_added() {
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(middleware::from_fn(add_clacks_header));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let header = response.headers().get("X-Clacks-Overhead");
        assert!(header.is_some());
        assert_eq!(header.unwrap(), "GNU Terry Pratchett");
    }
}
