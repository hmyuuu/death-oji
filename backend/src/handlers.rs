use crate::gugugaga::gugugagaClient;
use crate::models::*;
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

pub struct AppState {
    pub gugugaga_client: gugugagaClient,
}

pub async fn start_stream(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StartStreamRequest>,
) -> Result<Json<StreamInfo>, (StatusCode, Json<ErrorResponse>)> {
    // Start live stream
    let stream_info = state
        .gugugaga_client
        .start_live(&req.room_id, &req.cookies, &req.csrf_token, req.area_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to start stream: {}", e),
                }),
            )
        })?;

    Ok(Json(stream_info))
}

pub async fn stop_stream(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StopStreamRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state
        .gugugaga_client
        .stop_live(&req.room_id, &req.cookies, &req.csrf_token)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to stop stream: {}", e),
                }),
            )
        })?;

    Ok(StatusCode::OK)
}

pub async fn update_stream(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateStreamRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state
        .gugugaga_client
        .update_title(&req.room_id, &req.cookies, &req.csrf_token, &req.title)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to update stream: {}", e),
                }),
            )
        })?;

    Ok(StatusCode::OK)
}

pub async fn get_user_info(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserInfoRequest>,
) -> Result<Json<UserInfo>, (StatusCode, Json<ErrorResponse>)> {
    let user_info = state
        .gugugaga_client
        .get_user_info(&req.cookies)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to get user info: {}", e),
                }),
            )
        })?;

    Ok(Json(user_info))
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_partitions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let partitions = state.gugugaga_client.get_partitions().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to get partitions: {}", e),
            }),
        )
    })?;

    Ok(Json(partitions))
}

pub async fn generate_qrcode(
    State(state): State<Arc<AppState>>,
) -> Result<Json<QrCodeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let qr_data = state.gugugaga_client.generate_qrcode().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to generate QR code: {}", e),
            }),
        )
    })?;

    Ok(Json(qr_data))
}

pub async fn poll_qrcode(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QrPollRequest>,
) -> Result<Json<QrPollResponse>, (StatusCode, Json<ErrorResponse>)> {
    let poll_result = state
        .gugugaga_client
        .poll_qrcode(&req.qrcode_key)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to poll QR code: {}", e),
                }),
            )
        })?;

    Ok(Json(poll_result))
}
