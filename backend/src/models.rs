use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub room_id: String,
    pub cookies: String,
    pub csrf_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartStreamRequest {
    pub room_id: String,
    pub cookies: String,
    pub csrf_token: String,
    pub title: String,
    pub area_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopStreamRequest {
    pub room_id: String,
    pub cookies: String,
    pub csrf_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStreamRequest {
    pub room_id: String,
    pub cookies: String,
    pub csrf_token: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInfo {
    pub server_url: String,
    pub stream_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct gugugagaApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampData {
    pub now: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionData {
    pub build: i64,
    pub curr_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RtmpData {
    pub addr: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartLiveData {
    pub rtmp: RtmpData,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoRequest {
    pub cookies: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub uid: u64,
    pub uname: String,
    pub face: String,
    pub level: u32,
    pub coins: f64,
    pub bcoin: f64,
    pub current_exp: u64,
    pub next_exp: String,
    pub following: u64,
    pub follower: u64,
    pub dynamic_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrCodeData {
    pub url: String,
    pub qrcode_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrCodeResponse {
    pub qr_url: String,
    pub qrcode_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrPollRequest {
    pub qrcode_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrPollResponse {
    pub code: i32,
    pub message: String,
    pub cookies: Option<String>,
    pub csrf_token: Option<String>,
    pub room_id: Option<String>,
}
