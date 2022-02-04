use std::detect::__is_feature_detected::fma;
use poem::{
    get, handler, IntoResponse,
    listener::TcpListener, Route, Server, web::Path,
};
use tokio::{
    main as tokio_main
};
#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio_main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000")).run(app).await
}