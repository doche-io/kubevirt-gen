use actix_web::{App, HttpServer, middleware::Logger};
use clap::Parser;
use tokio;
pub mod routes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long)]
  port: u16,
  #[clap(short, long)]
  bind: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let args = Args::parse();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  HttpServer::new(|| App::new().service(routes::generate::vm).service(routes::echo::echo).wrap(Logger::default()))
    .bind((args.bind, args.port))?
    .run()
    .await
}
