use crate::models::*;
use reqwest::Client;
use std::collections::HashMap;

const APP_KEY: &str = "aae92bc66f3edfab";
const APP_SEC: &str = "af125a0d5279fd576c1b4418a3e8276d";

pub struct gugugagaClient {
    client: Client,
}

impl gugugagaClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn app_sign(&self, mut params: HashMap<String, String>) -> HashMap<String, String> {
        params.insert("appkey".to_string(), APP_KEY.to_string());

        let mut keys: Vec<_> = params.keys().cloned().collect();
        keys.sort();

        let query: Vec<String> = keys
            .iter()
            .map(|k| format!("{}={}", k, params.get(k).unwrap()))
            .collect();
        let query_string = query.join("&");

        let sign_string = format!("{}{}", query_string, APP_SEC);
        let sign = format!("{:x}", md5::compute(sign_string.as_bytes()));

        params.insert("sign".to_string(), sign);
        params
    }

    fn parse_cookies(&self, cookie_str: &str) -> HashMap<String, String> {
        cookie_str
            .split(';')
            .filter_map(|pair| {
                let parts: Vec<&str> = pair.trim().splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub async fn get_timestamp(&self) -> Result<i64, String> {
        let resp = self
            .client
            .get("https://api.bilibili.com/x/report/click/now")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<TimestampData> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!("API error: {}", api_resp.message));
        }

        Ok(api_resp.data.unwrap().now)
    }

    pub async fn get_version(&self, cookies: &str) -> Result<VersionData, String> {
        let ts = self.get_timestamp().await?;

        let mut params = HashMap::new();
        params.insert("system_version".to_string(), "2".to_string());
        params.insert("ts".to_string(), ts.to_string());

        let signed_params = self.app_sign(params);

        let cookie_map = self.parse_cookies(cookies);

        let resp = self
            .client
            .get("https://api.live.bilibili.com/xlive/app-blink/v1/liveVersionInfo/getHomePageLiveVersion")
            .query(&signed_params)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://link.bilibili.com/p/center/index")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<VersionData> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!("API error: {}", api_resp.message));
        }

        Ok(api_resp.data.unwrap())
    }

    pub async fn start_live(
        &self,
        room_id: &str,
        cookies: &str,
        csrf_token: &str,
        area_id: u32,
    ) -> Result<StreamInfo, String> {
        let version = self.get_version(cookies).await?;
        let ts = self.get_timestamp().await?;

        let mut params = HashMap::new();
        params.insert("room_id".to_string(), room_id.to_string());
        params.insert("platform".to_string(), "pc_link".to_string());
        params.insert("area_v2".to_string(), area_id.to_string());
        params.insert("backup_stream".to_string(), "0".to_string());
        params.insert("csrf_token".to_string(), csrf_token.to_string());
        params.insert("csrf".to_string(), csrf_token.to_string());
        params.insert("build".to_string(), version.build.to_string());
        params.insert("version".to_string(), version.curr_version);
        params.insert("ts".to_string(), ts.to_string());

        let signed_params = self.app_sign(params);

        let cookie_map = self.parse_cookies(cookies);

        let resp = self
            .client
            .post("https://api.live.bilibili.com/room/v1/Room/startLive")
            .form(&signed_params)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://link.bilibili.com/p/center/index")
            .header("Origin", "https://link.bilibili.com")
            .header("Cookie", cookies)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<StartLiveData> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!(
                "API error: {} (code: {})",
                api_resp.message, api_resp.code
            ));
        }

        let data = api_resp.data.unwrap();
        Ok(StreamInfo {
            server_url: data.rtmp.addr,
            stream_key: data.rtmp.code,
        })
    }

    pub async fn stop_live(
        &self,
        room_id: &str,
        cookies: &str,
        csrf_token: &str,
    ) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("room_id", room_id);
        params.insert("platform", "pc_link");
        params.insert("csrf_token", csrf_token);
        params.insert("csrf", csrf_token);

        let cookie_map = self.parse_cookies(cookies);

        let resp = self
            .client
            .post("https://api.live.bilibili.com/room/v1/Room/stopLive")
            .form(&params)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://link.bilibili.com/p/center/index")
            .header("Origin", "https://link.bilibili.com")
            .header("Cookie", cookies)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!("API error: {}", api_resp.message));
        }

        Ok(())
    }

    pub async fn update_title(
        &self,
        room_id: &str,
        cookies: &str,
        csrf_token: &str,
        title: &str,
    ) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("room_id", room_id);
        params.insert("platform", "pc_link");
        params.insert("title", title);
        params.insert("csrf_token", csrf_token);
        params.insert("csrf", csrf_token);

        let cookie_map = self.parse_cookies(cookies);

        let resp = self
            .client
            .post("https://api.live.bilibili.com/room/v1/Room/update")
            .form(&params)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://link.bilibili.com/p/center/index")
            .header("Origin", "https://link.bilibili.com")
            .header("Cookie", cookies)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!("API error: {}", api_resp.message));
        }

        Ok(())
    }

    pub async fn get_partitions(&self) -> Result<serde_json::Value, String> {
        let data = std::fs::read_to_string("partition.json")
            .map_err(|e| format!("Failed to read partition.json: {}", e))?;
        let json: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse partition.json: {}", e))?;
        Ok(json)
    }

    pub async fn get_user_info(&self, cookies: &str) -> Result<UserInfo, String> {
        let resp = self
            .client
            .get("https://api.bilibili.com/x/web-interface/nav")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
            .header("Referer", "https://www.bilibili.com/")
            .header("Cookie", cookies)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let api_resp: gugugagaApiResponse<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        if api_resp.code != 0 {
            return Err(format!("API error: {}", api_resp.message));
        }

        let data = api_resp.data.unwrap();

        let stat_resp = self
            .client
            .get("https://api.bilibili.com/x/web-interface/nav/stat")
            .header("User-Agent", "Mozilla/5.0")
            .header("Cookie", cookies)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let stat_api: gugugagaApiResponse<serde_json::Value> = stat_resp
            .json()
            .await
            .map_err(|e| format!("Parse failed: {}", e))?;

        let stat_data = stat_api.data.unwrap_or(serde_json::json!({}));

        Ok(UserInfo {
            uid: data["mid"].as_u64().unwrap_or(0),
            uname: data["uname"].as_str().unwrap_or("").to_string(),
            face: data["face"].as_str().unwrap_or("").to_string(),
            level: data["level_info"]["current_level"].as_u64().unwrap_or(0) as u32,
            coins: data["money"].as_f64().unwrap_or(0.0),
            bcoin: data["wallet"]["bcoin_balance"].as_f64().unwrap_or(0.0),
            current_exp: data["level_info"]["current_exp"].as_u64().unwrap_or(0),
            next_exp: data["level_info"]["next_exp"]
                .as_str()
                .unwrap_or("0")
                .to_string(),
            following: stat_data["following"].as_u64().unwrap_or(0),
            follower: stat_data["follower"].as_u64().unwrap_or(0),
            dynamic_count: stat_data["dynamic_count"].as_u64().unwrap_or(0),
        })
    }
}
