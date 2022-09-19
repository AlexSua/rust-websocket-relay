use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use super::websocket::{MyData, MyWs};

#[get("/{id}")]
async fn websocket_handler(
    id: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<MyData>,
) -> Result<HttpResponse, Error> {
    return ws::start(
        MyWs {
            channels: data.channels.clone(),
            remote_addr: None,
            room_id: id.to_string(),
        },
        &req,
        stream,
    );
}
