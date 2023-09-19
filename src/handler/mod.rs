use super::ws;
use uuid;
use super::Client;

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
  let user_id = body.user_id;
  let uuid = Uuid::new_v4().simple().to_string();

  register_client(uuid.clone(), user_id, clients).await;
  Ok(json(&RegisterResponse {
    url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
  }))
}

async fn register_client(id: String, user_id: usize, clients: Clients) {
  clients.lock().await.insert(
    id,
    Client {
      user_id,
      topics: vec![String::from("cats")],
      sender: None,
    },
  );
}

pub fn health_handler() -> impl Future<Output = Result<impl Reply>> {
    futures::future::ready(Ok(StatusCode::OK))
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
  let client = clients.lock().await.get(&id).cloned();
  match client {
    Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
    None => Err(warp::reject::not_found()),
  }
}
