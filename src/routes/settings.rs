use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::common::WebAppData;
use crate::config;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::response::OkResponse;
use crate::routes::API_VERSION;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&format!("/{API_VERSION}/settings"))
            .service(web::resource("").route(web::get().to(get)).route(web::post().to(update)))
            .service(web::resource("/name").route(web::get().to(site_name)))
            .service(web::resource("/public").route(web::get().to(get_public))),
    );
}

/// Get Settings
///
/// # Errors
///
/// This function will return an error if unable to get user from database.
pub async fn get(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    // check for user
    let user = app_data.auth.get_user_compact_from_request(&req).await?;

    // check if user is administrator
    if !user.administrator {
        return Err(ServiceError::Unauthorized);
    }

    let settings: tokio::sync::RwLockReadGuard<config::TorrustBackend> = app_data.cfg.settings.read().await;

    Ok(HttpResponse::Ok().json(OkResponse { data: &*settings }))
}

/// Get Public Settings
///
/// # Errors
///
/// This function should not return an error.
pub async fn get_public(app_data: WebAppData) -> ServiceResult<impl Responder> {
    let public_settings = app_data.cfg.get_public().await;

    Ok(HttpResponse::Ok().json(OkResponse { data: public_settings }))
}

/// Get Name of Website
///
/// # Errors
///
/// This function should not return an error.
pub async fn site_name(app_data: WebAppData) -> ServiceResult<impl Responder> {
    let settings = app_data.cfg.settings.read().await;

    Ok(HttpResponse::Ok().json(OkResponse {
        data: &settings.website.name,
    }))
}

/// Update the settings
///
/// # Errors
///
/// Will return an error if:
///
/// - There is no logged-in user.
/// - The user is not an administrator.
/// - The settings could not be updated because they were loaded from env vars.
pub async fn update(
    req: HttpRequest,
    payload: web::Json<config::TorrustBackend>,
    app_data: WebAppData,
) -> ServiceResult<impl Responder> {
    // check for user
    let user = app_data.auth.get_user_compact_from_request(&req).await?;

    // check if user is administrator
    if !user.administrator {
        return Err(ServiceError::Unauthorized);
    }

    let _ = app_data.cfg.update_settings(payload.into_inner()).await;

    let settings = app_data.cfg.settings.read().await;

    Ok(HttpResponse::Ok().json(OkResponse { data: &*settings }))
}
