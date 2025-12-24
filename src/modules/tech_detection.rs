use reqwest::Client;
use std::time::Duration;

pub async fn detect_technologies(target: &str, timeout_secs: u64) -> Vec<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap();
    
    let url = format!("http://{}", target);
    let resp = client.get(&url).send().await;
    
    match resp {
        Ok(response) => {
            let headers = response.headers();
            let mut technologies = Vec::new();
            
            // Check for common technology headers
            if let Some(server) = headers.get("Server") {
                if let Ok(server_str) = server.to_str() {
                    technologies.push(format!("Server: {}", server_str));
                }
            }
            
            if let Some(x_powered_by) = headers.get("X-Powered-By") {
                if let Ok(x_powered_by_str) = x_powered_by.to_str() {
                    technologies.push(format!("X-Powered-By: {}", x_powered_by_str));
                }
            }
            
            if let Some(x_generator) = headers.get("X-Generator") {
                if let Ok(x_generator_str) = x_generator.to_str() {
                    technologies.push(format!("X-Generator: {}", x_generator_str));
                }
            }
            
            // Get the body to check for content-based fingerprints
            let body = response.text().await.unwrap_or_default();
            
            // Check for common HTML meta tags
            if body.contains("<meta name=\"generator\" content=\"WordPress") {
                technologies.push("WordPress".to_string());
            }
            
            if body.contains("content=\"Joomla") {
                technologies.push("Joomla".to_string());
            }
            
            if body.contains("Drupal.settings") {
                technologies.push("Drupal".to_string());
            }
            
            // Check for common JavaScript libraries
            if body.contains("jquery") {
                technologies.push("jQuery".to_string());
            }
            
            if body.contains("react") {
                technologies.push("React".to_string());
            }
            
            if body.contains("angular") {
                technologies.push("Angular".to_string());
            }
            
            if body.contains("vue") {
                technologies.push("Vue.js".to_string());
            }
            
            technologies
        }
        Err(_) => Vec::new(),
    }
}
