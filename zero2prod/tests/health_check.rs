use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configurations::{get_configuration, DatabaseSettings};

async fn spawn_app() -> (String, PgPool) {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().to_string();
    let server =
        zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    (port, connection_pool)
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
    let mut connection = PgConnection::connect(&config.get_connection_string_without_db())
        .await
        .expect("Failed to connect to db");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create a database");

    // migrate database
    let connection_pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to db");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let (address, _) = spawn_app().await;
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
    let name = "MyAlpaca";
    let email = "myalpaca@test.com";
    let body = format!("name={}&email={}", name, email);

    let (address, connection_pool) = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send POST message");

    assert_eq!(200, response.status().as_u16());

    let query_result = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&connection_pool)
        .await
        .expect("Failed to fetch one record");

    assert_eq!(query_result.email, email);
    assert_eq!(query_result.name, name);
}

#[tokio::test]
async fn subscribe_return_a_400_with_invalid_form_data() {
    let (address, _) = spawn_app().await;
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
