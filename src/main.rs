use std::path::PathBuf;

use backend::{Component,SubRoutine, xmlparser};

mod backend;

#[derive(Default)]
struct App {
    components: Vec<Component>,
    routines: Vec<SubRoutine>
}

#[tokio::main]
async fn main() {
    println!("Hello, world!")
}