use futures::TryStreamExt;
use hyper::header::CONTENT_TYPE;
use hyper::http::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use rust_sm_slim::Payload;
use std::convert::Infallible;
use std::env;
use std::fmt::Display;
use std::net::SocketAddr;
use std::num::ParseIntError;

// http related constants
const PING_PATH: &str = "/ping";
const INVOCATION_PATH: &str = "/invocations";
const CUSTOM_ATTR_HEADER: &str = "X-Amzn-SageMaker-Custom-Attributes";
// https://www.asciitable.com/
const BYTES: &[u8; 25000000] = &[65 as u8; 25000000]; // 'A'.to_digit(10).unwrap() as u8;

const NULL: Option<usize> = None;
const PORT_ENV_VAR: &str = "SAGEMAKER_BIND_TO_PORT";

async fn serve(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.uri().path(), req.method()) {
        (PING_PATH, &Method::GET) => ping(req).await,
        (INVOCATION_PATH, &Method::POST) => invoke(req).await,
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())?;
            Ok(response)
        }
    }
}

async fn ping(_req: Request<Body>) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?;
    Ok(response)
}

fn bad_request<T: Display>(e: T) -> Result<Response<Body>, Error> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(format!("{}", e)))
}

async fn invoke(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (parts, body) = req.into_parts();
    // Drain request body
    if let Err(res) = body.try_fold(NULL, |v, _chunk| async move { Ok(v) }).await {
        return bad_request(res);
    }
    let custom_attr = parts.headers.get(CUSTOM_ATTR_HEADER);
    let payload = Payload::from_header(custom_attr);
    if let Err(e) = payload {
        return bad_request(e);
    }
    let payload = payload.unwrap();
    tokio::time::sleep(payload.sleep_time).await;
    let bytes = &BYTES[0..(payload.response_size as usize)];
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(bytes))?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let port: u16 = env::var(PORT_ENV_VAR)
        .map_err(|e| e.to_string())
        .and_then(|v| v.parse().map_err(|e: ParseIntError| e.to_string()))
        .map_err(|e| eprintln!("Error parsing env var '{}': {}", PORT_ENV_VAR, e))
        .unwrap_or(8080);
    println!("Using port {}", port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(serve))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
