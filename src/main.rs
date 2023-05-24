use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;

async fn handle_request(_: Request<Body>, num: usize) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(format!("Server #{}", num))))
}

async fn run_server(addr: SocketAddr, num: usize) {
    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| async move {
        Ok::<_, hyper::Error>(service_fn(move |req| async move {
            handle_request(req, num).await
        }))
    }));

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr2: SocketAddr = "127.0.0.1:8081".parse().unwrap();

    let task1 = tokio::spawn(run_server(addr1, 1));
    let task2 = tokio::spawn(run_server(addr2, 2));

    tokio::try_join!(task1, task2).unwrap();
}
