use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};
use crate::network::handler;
use crate::chain::Chain;

pub(crate) type Result<T> = std::result::Result<T, Rejection>;
pub(crate) type Clients = Arc<RwLock<HashMap<String, Client>>>;
pub(crate) type Blockchain = Arc<RwLock<Chain>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
pub async fn start_client(port: u16, blockchain: Chain) {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let chain = Arc::new(RwLock::new(blockchain));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_port(port))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_blockchain(chain.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_port(port: u16) -> impl Filter<Extract = (u16,), Error = Infallible> + Copy {
    warp::any().map(move ||port)
}

fn with_blockchain(chain: Blockchain) -> impl Filter<Extract = (Blockchain,), Error = Infallible> + Clone {
    warp::any().map(move ||chain.clone())
}
