use actix_web::{App, HttpServer};
use clap::Parser;
use tokio;
mod routes;

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
  HttpServer::new(|| App::new().service(routes::echo::echo))
    .bind((args.bind, args.port))?
    .run()
    .await
}
