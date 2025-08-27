use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;

use crate::models::Authentication;
use crate::security::verify_password;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn AuthRepo>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync {
    async fn find_user(&self, username: &str) -> Result<Option<Authentication>, anyhow::Error>;
}

pub async fn login_handler(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    if body.username.trim().is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "username and password are required"
        }));
    }

    let user = match state.repo.find_user(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "invalid credentials"
            }))
        }
        Err(e) => {
            log::error!("repo error: {e}");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal error"
            }));
        }
    };

    match verify_password(&body.password, &user.password) {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "username": user.username,
            "role": user.privalige
        })),
        Ok(false) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "invalid credentials"
        })),
        Err(e) => {
            log::warn!("bad hash for {}: {e}", user.username);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal error"
            }))
        }
    }
}

