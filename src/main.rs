//! A Hello World example application for working with Gotham.

extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde_json;

use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::{Body, HeaderMap, Method, Response, StatusCode, Uri, Version};
use serde::{Deserialize, Serialize};

//const HELLO_WORLD: &'static str = "Hello World!";

#[derive(Deserialize, Serialize, Debug)]
pub struct HelloRequest {
    pub msg: String
}

#[derive(Deserialize, Serialize)]
pub struct HelloResponse {
    pub msg: String
}

/// Extract the main elements of the request except for the `Body`
fn print_request_elements(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = Version::borrow_from(state);
    let headers = HeaderMap::borrow_from(state);
    println!("Method: {:?}", method);
    println!("URI: {:?}", uri);
    println!("HTTP Version: {:?}", http_version);
    println!("Headers: {:?}", headers);
}

/// Show the GET request components by printing them.
fn get_handler(state: State) -> (State, Response<Body>) {
//    print_request_elements(&state);
    let res = create_empty_response(&state, StatusCode::OK);

    (state, res)
}

/// Extracts the elements of the POST request and prints them
fn post_handler(mut state: State) -> Box<HandlerFuture> {
    print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                println!("Body: {}", body_content);
                let deserialized: HelloRequest = serde_json::from_str(&body_content).unwrap();
                println!("Deserialized body: {:?}", deserialized);
                let resp = HelloResponse { msg: "General Kenobi".to_string() };
                let res = create_response(
                    &state,
                    StatusCode::OK,
                    mime::APPLICATION_JSON,
                    serde_json::to_vec(&resp).unwrap(),
                );
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

/// Create a `Router`
fn router() -> Router {
    build_simple_router(|route| {
        route.associate("/", |assoc| {
            assoc.get().to(get_handler);
            assoc.post().to(post_handler);
        });
    })
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "0.0.0.0:80";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
*/
