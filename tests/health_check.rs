
#[actix_rt::test]
async fn health_check_works() {
   let address = spawn_app();
   
   let client = reqwest::Client::new();
   let resp = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(resp.status().is_success());
}


fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind on random local port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}