use trust_dns_resolver::TokioAsyncResolver;
use std::collections::HashSet;
use tokio::task;
use std::fs::File;
use std::io::BufRead;

pub async fn enumerate_subdomains(
    target: &str, 
    wordlist_path: Option<&str>, 
    _threads: usize
) -> Vec<String> {
    let wordlist = match wordlist_path {
        Some(path) => load_wordlist(path).unwrap_or_else(|_| get_default_wordlist()),
        None => get_default_wordlist(),
    };
    
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let mut found = HashSet::new();
    let mut tasks = Vec::new();
    
    for sub in wordlist {
        let target = target.to_string();
        let resolver = resolver.clone();
        tasks.push(task::spawn(async move {
            check_subdomain(&resolver, &target, &sub).await
        }));
    }
    
    for task in tasks {
        if let Ok(Some(subdomain)) = task.await {
            found.insert(subdomain);
        }
    }
    
    found.into_iter().collect()
}

async fn check_subdomain(resolver: &TokioAsyncResolver, target: &str, sub: &str) -> Option<String> {
    let domain = format!("{}.{}", sub, target);
    match resolver.lookup_ip(&domain).await {
        Ok(_) => Some(domain),
        Err(_) => None,
    }
}

fn get_default_wordlist() -> Vec<String> {
    vec![
        "www", "mail", "ftp", "admin", "blog", "dev", "test", "staging",
        "api", "app", "shop", "store", "news", "forum", "support", "docs"
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
