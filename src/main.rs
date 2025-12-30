use blog_engine::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Fatal error: {:?}", e);
        std::process::exit(1);
    }
}
