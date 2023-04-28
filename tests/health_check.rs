use reqwest::{Client, StatusCode};
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Run a server in the background on a random port, and return the server address.
async fn spawn_app(pool: PgPool) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = zero2prod::startup::run(listener, pool.clone()).expect("Failed to run server");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: pool,
    }
}

#[sqlx::test]
async fn health_check_works(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[sqlx::test]
async fn subscribe_returns_200_for_valid_form_data(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let client = Client::new();

    // Act
    let body = "name=le%20guin&email=ursual_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(StatusCode::OK, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription from DB");

    assert_eq!("ursual_le_guin@gmail.com", saved.email);
    assert_eq!("le guin", saved.name);
}

#[sqlx::test]
async fn subscribe_returns_400_when_data_is_missing(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "{}: POST /subscriptions responded {}",
            error_message,
            response.status()
        )
    }
}
