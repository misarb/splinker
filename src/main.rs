use splinker::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
