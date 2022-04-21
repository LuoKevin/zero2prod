use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("No random bindy work :(");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Server no runny :(");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Request no executey D:");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    //Setup
    let app_address =spawn_app();
    let configuration = get_configuration().expect("Goddamnit, cant get configuration.");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Poopy, can't connect to Postgres.");
    let client = reqwest::Client::new();

    //Exercise
    let body = "name=Jimmy%20Russels&email=jimmy%40aol.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("No can requesty :P");

    //Verify
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name from SUBSCRIPTIONS",)
        .fetch_one(&mut connection)
        .await
        .expect("Can't get subscriptions, poop!");

    assert_eq!(saved.email, "jimmy@aol.com");
    assert_eq!(saved.name, "Jimmy Russels");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
// Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
// Assert
        assert_eq!(
            400,
            response.status().as_u16(),
// Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
