use std::{
    fs,
    io::{self, Write},
};
use structopt::StructOpt;
use warp::Filter;
// #[tokio::main]
// async fn main() {
//     // GET /hello/warp => 200 OK with body "Hello, warp!"
//     let hello = warp::path!("hello" / String)
//         .map(|name| format!("Hello, {}!", name));

//     warp::serve(hello)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }
#[derive(StructOpt)]
struct Cli {
    pattern: String
}
fn main() {
    let agrs = Cli::from_args();
    let mut std_out = io::stdout();
    writeln!(std_out, "{}: hello", agrs.pattern);
}
