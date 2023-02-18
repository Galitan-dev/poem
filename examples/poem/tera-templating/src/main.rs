use poem::{
    ctx, get, handler,
    listener::TcpListener,
    web::Path,
    Route, Server,
    EndpointExt,
    tera::{TeraTemplating, TeraTemplate, Tera}
};

#[handler]
fn hello(Path(name): Path<String>, tera: Tera) -> TeraTemplate {
    tera.render("index.html.tera", &ctx!{ "name": &name })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(TeraTemplating::from_glob("templates/**/*"));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
