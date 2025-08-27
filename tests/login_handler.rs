use actix_web::{test, web, App};
use std::collections::HashMap;
use std::sync::Arc;

use auth::{login_handler, AppState, AuthRepo, LoginRequest};
use auth::models::Authentication;
use auth::security::hash_password;

struct MockRepo {
    // username -> (role, phc_hash)
    users: HashMap<String, (String, String)>,
}

#[async_trait::async_trait]
impl AuthRepo for MockRepo {
    async fn find_user(&self, username: &str) -> Result<Option<Authentication>, anyhow::Error> {
        if let Some((role, phc)) = self.users.get(username) {
            Ok(Some(Authentication {
                username: username.to_string(),
                created: chrono::Utc::now(),
                privilege: role.clone(),
                password: phc.clone(),
            }))
        } else {
            Ok(None)
        }
    }
}

#[actix_rt::test]
async fn test_login_ok() {
    let mut users = HashMap::new();
    let phc = hash_password("s3cret").unwrap();
    users.insert("alice".into(), ("user".into(), phc));
    let state = AppState { repo: Arc::new(MockRepo { users }) };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/login", web::post().to(login_handler))
    ).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&LoginRequest { username: "alice".into(), password: "s3cret".into() })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["username"], "alice");
    assert_eq!(body["role"], "user");
}

#[actix_rt::test]
async fn test_login_invalid() {
    let mut users = HashMap::new();
    let phc = hash_password("s3cret").unwrap();
    users.insert("alice".into(), ("user".into(), phc));
    let state = AppState { repo: Arc::new(MockRepo { users }) };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/login", web::post().to(login_handler))
    ).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&LoginRequest { username: "alice".into(), password: "wrong".into() })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

