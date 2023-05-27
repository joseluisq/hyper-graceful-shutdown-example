# Hyper graceful shutdown example for multiple HTTP servers

> A simple example demonstrating graceful shutdown for multiple Hyper HTTP servers using Tokio message passing channel. 

## Use

```sh
cargo run
# Waiting 5 secs before sending the signal...
# Server #2 is listening on 127.0.0.1:8081
# Server #2 is waiting for signal...
# Server #1 is listening on 127.0.0.1:8080
# Server #1 is waiting for signal...
# ---- 5 seconds elapsed ----
# Termination signal sent!
# Stopping server #2...
# Server #2 is done!
# Stopping server #1...
# Server #1 is done!
# All servers were shut down correctly!
```
