use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App};
use zero2prod::health_check;
fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // get the port number
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[actix_web::test]
async fn check_health_works() {
    // let app =
    //     test::init_service(App::new().service(web::resource("/healthz").to(health_check))).await;
    let addr = spawn_app();
    // Create request object
    // let req = test::TestRequest::with_uri("/healthz").to_request();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/healthz", &addr))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}
