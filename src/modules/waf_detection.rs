use reqwest::Client;
use std::time::Duration;

pub async fn detect_waf(target: &str, timeout_secs: u64) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap();
    
    let url = format!("http://{}", target);
    let resp = client.get(&url).send().await;
    
    match resp {
        Ok(response) => {
            let headers = response.headers();
            
            // Check for common WAF headers
            if let Some(waf_name) = detect_by_headers(headers) {
                return Some(waf_name);
            }
            
            // Check for common WAF cookies
            if let Some(waf_name) = detect_by_cookies(headers) {
                return Some(waf_name);
            }
            
            // Check response body for WAF fingerprints
            let body = response.text().await.unwrap_or_default();
            if let Some(waf_name) = detect_by_body(&body) {
                return Some(waf_name);
            }
            
            None
        }
        Err(_) => None,
    }
}

fn detect_by_headers(headers: &reqwest::header::HeaderMap) -> Option<String> {
    if let Some(server) = headers.get("Server") {
        if let Ok(server_str) = server.to_str() {
            if server_str.contains("cloudflare") {
                return Some("Cloudflare".to_string());
            }
            if server_str.contains("Sucuri") {
                return Some("Sucuri".to_string());
            }
        }
    }
    
    if let Some(x_waf) = headers.get("X-WAF") {
        if let Ok(waf_name) = x_waf.to_str() {
            return Some(waf_name.to_string());
        }
    }
    
    None
}

fn detect_by_cookies(headers: &reqwest::header::HeaderMap) -> Option<String> {
    if let Some(cookie) = headers.get("Set-Cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            if cookie_str.contains("__cfduid") {
                return Some("Cloudflare".to_string());
            }
            if cookie_str.contains("sucuri_cloudproxy_uuid") {
                return Some("Sucuri".to_string());
            }
        }
    }
    
    None
}

fn detect_by_body(body: &str) -> Option<String> {
    if body.contains("cloudflare-nginx") {
        return Some("Cloudflare".to_string());
    }
    if body.contains("sucuri") {
        return Some("Sucuri".to_string());
    }
    if body.contains("ModSecurity") {
        return Some("ModSecurity".to_string());
    }
    
    None
}
