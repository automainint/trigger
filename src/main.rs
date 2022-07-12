use std::env;
use std::process::Command;
use warp::*;

async fn run_action(action: String) -> Result<impl Reply, Rejection> {
  let _ = Command::new("sh").arg(action).output();
  Ok(reply())
}

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  println!("POST http://127.0.0.1:{}/{} will execute {}", args[1], args[2], args[3]);

  let port: u16     = args[1].parse().unwrap();
  let name: String  = args[2].clone();
  let action        = args[3].clone();

  serve(
    path(name)
      .and(post())
      .and(any().map(move || action.clone()))
      .and_then(run_action))
    .run(([0, 0, 0, 0], port))
    .await;
}
