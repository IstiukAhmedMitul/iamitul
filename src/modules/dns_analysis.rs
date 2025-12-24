use trust_dns_resolver::TokioAsyncResolver;
use std::collections::HashMap;
use tokio::task;
use trust_dns_resolver::proto::rr::RecordType;
use std::fs::File;
use std::io::BufRead;

pub async fn get_dns_records(target: &str) -> HashMap<String, Vec<String>> {
    let record_types = vec![
        RecordType::A, 
        RecordType::AAAA, 
        RecordType::MX, 
        RecordType::NS, 
        RecordType::TXT, 
        RecordType::SOA, 
        RecordType::CNAME,
        RecordType::PTR,
        RecordType::SRV,
        RecordType::DNSKEY,
        RecordType::DS,
    ];
    
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(e) => {
            eprintln!("Failed to create DNS resolver: {}", e);
            return HashMap::new();
        }
    };
    
    let mut records_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut tasks = Vec::new();
    
    for record_type in record_types {
        let target_clone = target.to_string(); // Clone the target for each task
        let resolver = resolver.clone();
        tasks.push(task::spawn(async move {
            println!("Querying {} records for {}", record_type, target_clone);
            let result = resolver.lookup(target_clone.clone(), record_type, Default::default()).await;
            match result {
                Ok(lookup) => {
                    let records: Vec<String> = lookup
                        .iter()
                        .map(|r| format!("{}", r))
                        .collect();
                    
                    if !records.is_empty() {
                        println!("Found {} {} records: {:?}", records.len(), record_type, records);
                    } else {
                        println!("No {} records found for {}", record_type, target_clone);
                    }
                    
                    Some((format!("{:?}", record_type), records))
                }
                Err(_) => {
                    // Silently ignore failed queries
                    None
                }
            }
        }));
    }
    
    for task in tasks {
        if let Ok(Some((record_type, records))) = task.await {
            if !records.is_empty() {
                records_map.insert(record_type, records);
            }
        }
    }
    
    records_map
}

pub async fn enumerate_subdomains_with_wordlist(
    target: &str, 
    wordlist_path: Option<&str>
) -> Vec<String> {
    let wordlist = match wordlist_path {
        Some(path) => load_wordlist(path).unwrap_or_else(|_| get_default_subdomain_wordlist()),
        None => get_default_subdomain_wordlist(),
    };
    
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(e) => {
            eprintln!("Failed to create DNS resolver: {}", e);
            return Vec::new();
        }
    };
    
    let mut found = Vec::new();
    let mut tasks = Vec::new();
    
    for subdomain in wordlist {
        let target_clone = target.to_string();
        let subdomain = format!("{}.{}", subdomain, target_clone);
        let resolver = resolver.clone();
        tasks.push(task::spawn(async move {
            let result = resolver.lookup_ip(subdomain.clone()).await;
            match result {
                Ok(_) => {
                    println!("Found subdomain: {}", subdomain);
                    Some(subdomain)
                }
                Err(_) => None,
            }
        }));
    }
    
    for task in tasks {
        if let Ok(Some(subdomain)) = task.await {
            found.push(subdomain);
        }
    }
    
    found
}

fn get_default_subdomain_wordlist() -> Vec<String> {
    vec![
        "www", "mail", "ftp", "admin", "blog", "dev", "test", "staging",
        "api", "app", "shop", "store", "news", "forum", "support", "docs",
        "cdn", "media", "static", "images", "files", "download", "video",
        "audio", "book", "books", "contact", "about", "jobs", "careers",
        "partners", "investors", "press", "news", "events", "help", "service",
        "services", "hr", "humanresources", "finance", "accounting", "legal",
        "it", "informationtechnology", "tech", "technology", "research",
        "development", "rd", "innovation", "lab", "labs", "qa", "qualityassurance",
        "testing", "staging", "production", "prod", "backup", "backup1", "backup2",
        "db", "database", "sql", "mysql", "postgres", "oracle", "mongodb", "redis",
        "cache", "search", "elasticsearch", "solr", "lucene", "analytics",
        "metrics", "monitoring", "logging", "log", "logs", "error", "errors",
        "report", "reports", "export", "import", "sync", "cron", "job", "jobs",
        "task", "tasks", "queue", "worker", "bot", "robot", "crawler", "spider",
        "scan", "scanner", "security", "auth", "authentication", "login",
        "logout", "register", "signup", "signin", "signout", "password", "reset",
        "forgot", "recover", "verify", "confirm", "activate", "enable", "disable",
        "block", "unblock", "ban", "unban", "delete", "remove", "add", "create",
        "update", "edit", "modify", "save", "submit", "send", "post", "get",
        "put", "patch", "delete", "head", "options", "trace", "connect"
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
