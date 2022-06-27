use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{configure_database, get_configuration};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

#[actix_rt::test]
async fn health_check_works() {
   let app = spawn_app().await;
   
   let client = reqwest::Client::new();
   let resp = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn subscribe_return_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=td_bn&email=td_bn%40gmail.com";

    // Connect to database
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&*connection_string)
        .await
        .expect("Failed to connect to database");

    let resp = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute post request!");

    assert_eq!(resp.status().as_u16(), 200, "Valid form data subscription didn't return OK status");
    let user = sqlx::query!("SELECT email, name from subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!("td_bn", user.name);
    assert_eq!("td_bn@gmail.com", user.email);
}

#[actix_rt::test]
async fn subscribe_returns_400_when_data_is_invalid() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=tdbn", "missing email"),
        ("email=td-bn@tdbn.com", "missing name"),
        ("", "missing email and name"),
    ];

    for (body, message) in test_cases {
        let resp = client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute post request!");

        assert_eq!(
            resp.status().as_u16(),
            400,
            "API didn't fail with 400 when payload was {}", message
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind on random local port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("Failed to read configuration");
    // Randomize db name for integration tests
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool
    }
}