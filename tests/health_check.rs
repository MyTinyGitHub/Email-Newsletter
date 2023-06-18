use zero2prod::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() { 
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20gui&email=ursula_le_guin%40gmail.com";
    response = client
        .post(&format!("{}/subscribtion", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() { 
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email");
    ]

    for(invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribtion", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), error_message);
    }
}

fn spawn_app() {
    let listener = TcpListener::bind(127.0.0.1:0).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(lisener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("127.0.0.1:{}", port)
}
