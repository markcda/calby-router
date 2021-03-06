use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
pub async fn main() -> MResult<()> {
  dotenv()?;
  let hyper_addr: SocketAddr = env::var("HYPER_ADDR")?.parse()?;
  let service = hyper::service::make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
    let env_variables = env_variables.clone();
    let addr = conn.remote_addr();
    let service = hyper::service::service_fn(move |req| {
      hyper_router::router(req, env_variables.clone(), addr)
    });
    async move { Ok::<_, std::convert::Infallible>(service) }
  });
  let server = hyper::Server::bind(&hyper_addr).serve(service);
  println!("Сервер слушает по адресу http://{}", hyper_addr);
  let finisher = server.with_graceful_shutdown(hyper_router::shutdown());
  match finisher.await {
    Err(e) => eprintln!("Ошибка сервера: {}", e),
    _ => println!("\nСервер успешно выключен."),
  };
  Ok(())
}
