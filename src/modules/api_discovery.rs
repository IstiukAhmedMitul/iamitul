use reqwest::Client;
use std::collections::HashSet;
use tokio::task;
use std::fs::File;
use std::io::BufRead;
use std::time::Duration;

pub async fn discover_api_endpoints(
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
    
    // Also check common API paths
    let api_paths = vec![
        format!("http://{}/api", target),
        format!("http://{}/api/v1", target),
        format!("http://{}/api/v2", target),
        format!("http://{}/rest", target),
        format!("http://{}/graphql", target),
        format!("https://{}/api", target),
        format!("https://{}/api/v1", target),
        format!("https://{}/api/v2", target),
        format!("https://{}/rest", target),
        format!("https://{}/graphql", target),
    ];
    
    // Check API paths first
    for api_path in &api_paths {
        let api_path = api_path.clone();
        let client = client.clone();
        tasks.push(task::spawn(async move {
            check_api_endpoint(&client, &api_path, "").await
        }));
    }
    
    // Then check wordlist endpoints
    for endpoint in wordlist {
        for base_url in &base_urls {
            let base_url = base_url.clone();
            let client = client.clone();
            let endpoint_clone = endpoint.clone(); // Clone the endpoint for each task
            tasks.push(task::spawn(async move {
                check_api_endpoint(&client, &base_url, &endpoint_clone).await
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

async fn check_api_endpoint(client: &Client, base_url: &str, endpoint: &str) -> Option<String> {
    // Ensure endpoint starts with a slash
    let endpoint = if endpoint.starts_with('/') {
        endpoint.to_string()
    } else {
        format!("/{}", endpoint)
    };
    
    let url = format!("{}{}", base_url, endpoint);
    
    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            
            // Only show status for non-404 responses
            if status != reqwest::StatusCode::NOT_FOUND {
                println!("Checking API: {} - Status: {}", url, status);
            }
            
            // Consider 200, 201, 301, 302, 307, and 403 as valid responses
            if status.is_success() || status.is_redirection() || status == reqwest::StatusCode::FORBIDDEN {
                // For 403 Forbidden, we don't need to check content type - it's definitely an endpoint
                if status == reqwest::StatusCode::FORBIDDEN {
                    println!("Found API endpoint (Forbidden): {}", url);
                    return Some(url);
                }
                
                // For other responses, check if it's actually an API endpoint
                match response.headers().get("content-type") {
                    Some(content_type) => {
                        let content_type = content_type.to_str().unwrap_or("");
                        
                        // Only show content type for non-404 responses
                        if status != reqwest::StatusCode::NOT_FOUND {
                            println!("Content-Type: {}", content_type);
                        }
                        
                        // Check for API-related content types
                        if content_type.contains("json") || 
                           content_type.contains("xml") || 
                           content_type.contains("api") ||
                           content_type.contains("application") {
                            println!("Found API endpoint (Content-Type): {}", url);
                            return Some(url);
                        }
                        
                        // Also check if the URL contains API-related keywords
                        if url.to_lowercase().contains("api") ||
                           url.to_lowercase().contains("graphql") ||
                           url.to_lowercase().contains("rest") ||
                           url.to_lowercase().contains("swagger") ||
                           url.to_lowercase().contains("openapi") {
                            println!("Found API endpoint (URL pattern): {}", url);
                            return Some(url);
                        }
                        
                        // For 200 OK responses, even if they don't match other criteria, 
                        // they might still be API endpoints so we'll include them
                        if status.is_success() {
                            println!("Found API endpoint (200 OK): {}", url);
                            return Some(url);
                        }
                    }
                    None => {
                        // If no content type, but the response is successful, it might be an API endpoint
                        if status.is_success() {
                            println!("Found API endpoint (no content type): {}", url);
                            return Some(url);
                        }
                    }
                }
            }
            
            None
        }
        Err(_) => {
            // Silently ignore all errors (including timeouts)
            None
        }
    }
}

fn get_default_wordlist() -> Vec<String> {
    vec![
        "api", "v1", "v2", "v3", "graphql", "rest", "swagger.json",
        "api.json", "api-docs", "swagger", "openapi", "docs", "api/v1",
        "api/v2", "api/v3", "rest/api", "graphql/api", "api/graphql",
        "wp-json", "json", "xml", "rpc", "soap", "wsdl", "wadl",
        "users", "user", "admin", "login", "register", "auth", "oauth",
        "token", "refresh", "session", "logout", "profile", "settings",
        "config", "configuration", "health", "status", "ping", "info",
        "version", "metrics", "stats", "monitoring", "logs", "error",
        "data", "database", "db", "sql", "query", "search", "find",
        "create", "update", "delete", "patch", "put", "post", "get",
        "head", "options", "trace", "connect", "endpoint", "endpoints",
        "resource", "resources", "collection", "collections", "item", "items",
        "list", "lists", "page", "pages", "pagination", "filter", "sort",
        "order", "limit", "offset", "count", "total", "size", "format",
        "json", "xml", "csv", "pdf", "export", "import", "upload", "download",
        "file", "files", "image", "images", "document", "documents", "media",
        "video", "audio", "stream", "streaming", "live", "realtime", "websocket",
        "ws", "wss", "socket", "sockets", "event", "events", "webhook", "webhooks",
        "notification", "notifications", "email", "sms", "push", "message", "messages",
        "chat", "comment", "comments", "post", "posts", "article", "articles",
        "blog", "news", "feed", "rss", "atom", "sitemap", "robots", "favicon",
        "icon", "logo", "banner", "advertisement", "ads", "analytics", "tracking",
        "pixel", "beacon", "tag", "tags", "category", "categories", "tag", "tags",
        "product", "products", "catalog", "cart", "order", "orders", "payment",
        "checkout", "invoice", "receipt", "transaction", "transactions", "billing",
        "shipping", "delivery", "tax", "vat", "currency", "price", "discount",
        "coupon", "coupons", "promotion", "promotions", "offer", "offers", "deal",
        "deals", "sale", "sales", "customer", "customers", "user", "users", "member",
        "members", "account", "accounts", "subscription", "subscriptions", "plan",
        "plans", "package", "packages", "tier", "tiers", "level", "levels", "rank",
        "ranks", "point", "points", "reward", "rewards", "badge", "badges",
        "achievement", "achievements", "leaderboard", "leaderboards", "tournament",
        "tournaments", "competition", "competitions", "challenge", "challenges",
        "quiz", "quizzes", "survey", "surveys", "poll", "polls", "vote", "votes",
        "feedback", "review", "reviews", "rating", "ratings", "star", "stars",
        "like", "likes", "dislike", "dislikes", "favorite", "favorites", "bookmark",
        "bookmarks", "share", "shares", "social", "facebook", "twitter", "instagram",
        "linkedin", "youtube", "tiktok", "snapchat", "pinterest", "reddit", "discord",
        "slack", "teams", "zoom", "skype", "telegram", "whatsapp", "signal", "viber",
        "wechat", "line", "kakao", "naver", "daum", "yahoo", "bing", "google", "apple",
        "microsoft", "amazon", "netflix", "spotify", "youtube", "tiktok", "instagram",
        "facebook", "twitter", "linkedin", "pinterest", "reddit", "discord", "slack",
        "teams", "zoom", "skype", "telegram", "whatsapp", "signal", "viber", "wechat",
        "line", "kakao", "naver", "daum", "yahoo", "bing", "google", "apple", "microsoft",
        "amazon", "netflix", "spotify", "youtube", "tiktok", "instagram", "facebook",
        "twitter", "linkedin", "pinterest", "reddit", "discord", "slack", "teams",
        "zoom", "skype", "telegram", "whatsapp", "signal", "viber", "wechat", "line"
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
