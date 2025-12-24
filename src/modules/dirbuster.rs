use reqwest::Client;
use std::collections::HashSet;
use tokio::task;
use std::fs::File;
use std::io::BufRead;
use std::time::Duration;

pub async fn brute_directories(
    target: &str, 
    wordlist_path: Option<&str>, 
    _threads: usize, 
    timeout_secs: u64
) -> Vec<String> {
    let wordlist = match wordlist_path {
        Some(path) => load_wordlist(path).unwrap_or_else(|_| get_default_wordlist()),
        None => get_default_wordlist(),
    };
    
    // Create a client with redirect following and reasonable timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent("IAMitul/0.1.0")
        .build()
        .unwrap_or_else(|_| Client::new());
    
    let mut found = HashSet::new();
    let mut tasks = Vec::new();
    
    // Check both HTTP and HTTPS
    let base_urls = vec![
        format!("http://{}", target),
        format!("https://{}", target),
    ];
    
    for path in wordlist {
        for base_url in &base_urls {
            let base_url = base_url.clone();
            let client = client.clone();
            let path_clone = path.clone(); // Clone the path for each task
            tasks.push(task::spawn(async move {
                check_directory(&client, &base_url, &path_clone).await
            }));
        }
    }
    
    for task in tasks {
        if let Ok(Some(url)) = task.await {
            found.insert(url);
        }
    }
    
    found.into_iter().collect()
}

async fn check_directory(client: &Client, base_url: &str, path: &str) -> Option<String> {
    // Ensure path starts with a slash
    let path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };
    
    let url = format!("{}{}", base_url, path);
    
    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            
            // Only show status for non-404 responses
            if status != reqwest::StatusCode::NOT_FOUND {
                println!("Checking: {} - Status: {}", url, status);
            }
            
            // Consider 200, 201, 301, 302, 307, and 403 as valid responses
            if status.is_success() || status.is_redirection() || status == reqwest::StatusCode::FORBIDDEN {
                // For 403 Forbidden, we don't need to check content type - it's definitely a directory
                if status == reqwest::StatusCode::FORBIDDEN {
                    println!("Found directory (Forbidden): {}", url);
                    return Some(url);
                }
                
                // For other responses, check if it's actually a directory by looking at the content type
                match response.headers().get("content-type") {
                    Some(content_type) => {
                        let content_type = content_type.to_str().unwrap_or("");
                        
                        // Only show content type for non-404 responses
                        if status != reqwest::StatusCode::NOT_FOUND {
                            println!("Content-Type: {}", content_type);
                        }
                        
                        // If it's HTML, it's likely a directory
                        if content_type.contains("html") || content_type.contains("text") {
                            println!("Found directory (HTML): {}", url);
                            return Some(url);
                        }
                        
                        // Also check if the URL ends with a slash, which is a good indicator of a directory
                        if url.ends_with('/') {
                            println!("Found directory (ends with slash): {}", url);
                            return Some(url);
                        }
                        
                        // For 200 OK responses, even if they don't match other criteria, 
                        // they might still be directories so we'll include them
                        if status.is_success() {
                            println!("Found directory (200 OK): {}", url);
                            return Some(url);
                        }
                    }
                    None => {
                        // If no content type, but the response is successful, it might be a directory
                        if status.is_success() {
                            println!("Found directory (no content type): {}", url);
                            return Some(url);
                        }
                    }
                }
            }
            
            None
        }
        Err(e) => {
            // Only show non-timeout errors
            if !e.is_timeout() && !e.is_connect() {
                println!("Error checking {}: {}", url, e);
            }
            None
        }
    }
}

fn get_default_wordlist() -> Vec<String> {
    vec![
        "admin", "login", "wp-admin", "phpmyadmin", "dashboard",
        "config", "test", "backup", "api", "docs", "adminer",
        "console", "manager", "dev", "staging", "cgi-bin",
        "uploads", "images", "js", "css", "vendor"
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn load_wordlist(path: &str) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut wordlist = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() && !line.starts_with('#') {
            wordlist.push(line);
        }
    }
    
    Ok(wordlist)
}
