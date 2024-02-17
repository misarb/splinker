#[tokio::test]
async fn health_check_works() {
    //spawn our app
    spawn_app();
    // creat a requesst to do our test
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("failt to execut requesst.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = splinker::run().expect("Failed to run the Server");

    let _ = tokio::spawn(server);
}
