use std::convert::Infallible;
use std::future::Future;
use std::net::ToSocketAddrs;
use ruma::events::AnyEphemeralRoomEvent;
use ruma::serde::Raw;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body::{aggregate, Buf}, header, Body, Request, Response, StatusCode, Uri};
use rand::distributions::Alphanumeric;
use ruma::ServerName;
use serde::{Deserialize, Serialize};

use serde_json::value::to_raw_value;

/// Listen on `addrs` for incoming events, and use the given `handler` to handle those events.
pub async fn serve<S, F, R>(addrs: S, handler: F) -> Result<(), hyper::Error>
where
    S: ToSocketAddrs,
    F: Fn(String, Vec<Raw<AnyEphemeralRoomEvent>>) -> R + Sync + Send + Clone + 'static,
    R: Future<Output = Result<String, Infallible>> + Send,
{
    let service = make_service_fn(move |_| {
        let handler = handler.clone();
        async {
            let f = service_fn(move |req: Request<Body>| {
                let handler = handler.clone();
                async move {
                    let (parts, body) = req.into_parts();

                    // skip "/transactions/"
                    let txn_id = parts.uri.path()[14..].to_string();

                    let body = aggregate(body).await.unwrap();
                    let json: serde_json::Value = serde_json::from_reader(body.reader()).unwrap();

                    let events: Vec<Raw<AnyEphemeralRoomEvent>> = json
                        .as_object()
                        .unwrap()
                        .get("events")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|e| {
                            let raw_value = to_raw_value(e).unwrap();
                            Raw::from_json(raw_value)
                        })
                        .collect();

                    match handler(txn_id, events).await {
                        Err(_) => {} // TODO
                        Ok(_) => {}
                    }

                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from("{}"))
                        .unwrap();
                    Ok::<_, Infallible>(response)
                }
            });

            Ok::<_, Infallible>(f)
        }
    });

    let addr = addrs.to_socket_addrs().unwrap().next().unwrap();
    let server = Server::bind(&addr).serve(service);

    server.await
}

pub fn random_alphanumeric(n_chars: usize) -> String {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(n_chars)
        .map(char::from)
        .collect()
}

/// A struct containing information required by an application service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationService {
    server_name: Box<ServerName>,
    server_url: String,
}

impl ApplicationService {
    /// Create a new ApplicationService struct with the given information.
    pub fn new(server_name: Box<ServerName>, server_url: Uri) -> Self {
        Self {
            server_name,
            server_url: server_url.to_string(),
        }
    }

    /// Get a reference to the server name in this ApplicationService instance.
    pub fn server_name(&self) -> &ServerName {
        self.server_name.as_ref()
    }
    /// Get a reference to the server url in this ApplicationService instance.
    pub fn server_url(&self) -> Uri {
        self.server_url.parse().unwrap()
    }
}