use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().to_string();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    port
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to exectue request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=MyAlpaca&email=myalpaca@test.com";
    let response = client
        .post(&format!("http://{}/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send POST message");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_a_400_with_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=MyAlpaca", "missing email"),
        ("email=myalpaca@test.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, err_msg) in test_cases {
        let response = client
            .post(&format!("http://{}/subscribe", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send POST message");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The endpoint return 400 Bad Request due to {}",
            err_msg
        );
    }
}
