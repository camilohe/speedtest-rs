use rocket::data::{Data, ToByteUnit};
use rocket::http::{Header, Status};
use rocket::tokio::io;

#[derive(Responder)]
#[response(status = 200, content_type = "plain")]
pub struct EmptyResponder {
    inner: (),
    connection: Header<'static>,
}

#[post("/empty", data = "<data>")]
pub async fn empty(data: Data<'_>) -> EmptyResponder {
    match data.open(1.gigabytes()).stream_to(io::sink()).await {
        Ok(_) => print!(""),
        Err(_) => print!(""),
    };

    EmptyResponder {
        inner: (),
        connection: Header::new("Connection", "keep-alive"),
    }
}

#[options("/empty")]
pub async fn empty_options() -> Status {
    Status::NoContent
}

#[post("/backend/empty", data = "<data>")]
pub async fn backend_empty(data: Data<'_>) -> EmptyResponder {
    empty(data).await
}

#[get("/empty")]
pub async fn get_empty() -> Status {
    Status::Ok
}

#[post("/empty.php", data = "<data>")]
pub async fn empty_php(data: Data<'_>) -> EmptyResponder {
    empty(data).await
}

#[post("/backend/empty.php", data = "<data>")]
pub async fn backend_empty_php(data: Data<'_>) -> EmptyResponder {
    empty(data).await
}

#[get("/backend/empty.php")]
pub async fn get_backend_empty_php() -> Status {
    get_empty().await
}
