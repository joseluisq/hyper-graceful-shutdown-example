use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::{channel, Receiver};
use tokio::sync::Mutex;

pub type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;

async fn handle_request(_: Request<Body>, num: usize) -> Result<Response<Body>> {
    Ok(Response::new(Body::from(format!("Server #{}", num))))
}

async fn run_server(addr: SocketAddr, num: usize, receiver: Arc<Mutex<Option<Receiver<()>>>>) {
    let server = Server::bind(&addr)
        .serve(make_service_fn(move |_conn| async move {
            Ok::<_, hyper::Error>(service_fn(move |req| async move {
                handle_request(req, num).await
            }))
        }))
        // Graceful shutdown
        .with_graceful_shutdown(async {
            println!("Server #{} is waiting for signal...", num);
            if let Some(receiver) = &mut *receiver.lock().await {
                receiver.await.ok();
                println!("Stopping server #{}...", num);
            }
        });

    println!("Server #{} is listening on {}", num, addr);
    if let Err(e) = server.await {
        eprintln!("Server #{} error: {}", num, e);
    }
    println!("Server #{} is done!", num);
}

fn main() -> Result {
    // Our message passing
    // single-producer, single consumer channel
    let (sender, receiver) = channel::<()>();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            // Simuate signal sending
            let signal = tokio::spawn(async {
                // Wait 5 secs and send a signal
                println!("Waiting 5 secs for to send the signal...");
                std::thread::sleep(std::time::Duration::new(5, 0));
                println!("Termination signal sent!");
                let _ = sender.send(());
            });

            // Server 1
            let receiver1 = Arc::new(Mutex::new(Some(receiver)));
            let receiver2 = receiver1.clone();

            let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
            let task1 = tokio::spawn(run_server(addr1, 1, receiver1));

            // Server 2
            let addr2: SocketAddr = "127.0.0.1:8081".parse().unwrap();
            let task2 = tokio::spawn(run_server(addr2, 2, receiver2));

            // Join all concurrent branches
            if let Err(err) = tokio::try_join!(signal, task1, task2) {
                eprintln!("server failed to start up: {:?}", err);
            }

            println!("All servers were shut down correctly!");
        });

    Ok(())
}
