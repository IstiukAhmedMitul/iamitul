use clap::Parser;
use std::time::Instant;
use std::collections::HashMap;
use serde::Serialize;
use futures::future::BoxFuture;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

mod cli;
mod modules;
mod utils;

use modules::port_scanner;
use modules::dirbuster;
use modules::api_discovery;
use modules::subdomain_enum;
use modules::dns_analysis;
use modules::tech_detection;
use modules::ssl_analysis;
use modules::waf_detection;

use utils::output;

#[derive(Parser)]
#[clap(name = "IAMitul")]
#[clap(about = "High-performance reconnaissance tool")]
#[clap(version = "0.1.0")]
struct Args {
    #[clap(help = "Target domain or IP")]
    target: String,
    
    #[clap(short, long, help = "Output format (text, json, html)", default_value = "text")]
    output: String,
    
    #[clap(short, long, help = "Enable verbose output")]
    verbose: bool,
    
    #[clap(short, long, help = "Threads to use (default: auto)", default_value = "0")]
    threads: usize,
    
    #[clap(short, long, help = "Timeout in seconds (default: 10)", default_value = "10")]
    timeout: u64,
    
    #[clap(short, long, help = "Custom wordlist path")]
    wordlist: Option<String>,
    
    #[clap(short, long, help = "Custom DNS subdomain wordlist path")]
    dns_wordlist: Option<String>,
    
    #[clap(short, long, help = "Enable all modules")]
    all: bool,
    
    #[clap(long, help = "Enable port scanning")]
    ports: bool,
    
    #[clap(long, help = "Enable directory brute-forcing")]
    dirs: bool,
    
    #[clap(long, help = "Enable API discovery")]
    api: bool,
    
    #[clap(long, help = "Enable subdomain enumeration")]
    subdomains: bool,
    
    #[clap(long, help = "Enable DNS analysis")]
    dns: bool,
    
    #[clap(long, help = "Enable technology detection")]
    tech: bool,
    
    #[clap(long, help = "Enable SSL analysis")]
    ssl: bool,
    
    #[clap(long, help = "Enable WAF detection")]
    waf: bool,
}

#[derive(Serialize)]
struct ReconResult {
    target: String,
    timestamp: String,
    scan_duration: String,
    ports: Vec<port_scanner::PortResult>,
    directories: Vec<String>,
    api_endpoints: Vec<String>,
    subdomains: Vec<String>,
    dns_records: HashMap<String, Vec<String>>,
    technologies: Vec<String>,
    ssl_info: Option<ssl_analysis::SslInfo>,
    waf_detected: Option<String>,
    scan_options: ScanOptions,
}

#[derive(Serialize)]
struct ScanOptions {
    threads: usize,
    timeout: u64,
    modules_enabled: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("🔍 IAMitul - High-Performance Reconnaissance Tool");
    println!("📡 Target: {}", args.target);
    println!("⚡ Starting scan...\n");
    
    let start = Instant::now();
    
    // Determine which modules to run
    let modules_enabled = determine_modules(&args);
    
    // Always run technology detection if any module is enabled
    let run_tech = args.tech || args.all || modules_enabled.len() > 0;
    
    // Create shared data structures
    let results = Arc::new(Mutex::new(ReconResult {
        target: args.target.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        scan_duration: String::new(),
        ports: Vec::new(),
        directories: Vec::new(),
        api_endpoints: Vec::new(),
        subdomains: Vec::new(),
        dns_records: HashMap::new(),
        technologies: Vec::new(),
        ssl_info: None,
        waf_detected: None,
        scan_options: ScanOptions {
            threads: args.threads,
            timeout: args.timeout,
            modules_enabled: modules_enabled.clone(),
        },
    }));
    
    // Create a vector of tasks
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    
    // Add tasks for each enabled module
    if args.ports || args.all {
        let target = args.target.clone();
        let threads = args.threads;
        let timeout = args.timeout;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let ports = port_scanner::scan_ports(&target, threads, timeout).await;
            let mut results = results.lock().unwrap();
            results.ports = ports;
        }));
    }
    
    if args.dirs || args.all {
        let target = args.target.clone();
        let wordlist = args.wordlist.clone();
        let threads = args.threads;
        let timeout = args.timeout;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let dirs = dirbuster::brute_directories(&target, wordlist.as_deref(), threads, timeout).await;
            let mut results = results.lock().unwrap();
            results.directories = dirs;
        }));
    }
    
    if args.api || args.all {
        let target = args.target.clone();
        let wordlist = args.wordlist.clone();
        let threads = args.threads;
        let timeout = args.timeout;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let api = api_discovery::discover_api_endpoints(&target, wordlist.as_deref(), threads, timeout).await;
            let mut results = results.lock().unwrap();
            results.api_endpoints = api;
        }));
    }
    
    if args.subdomains || args.all {
        let target = args.target.clone();
        let wordlist = args.wordlist.clone();
        let threads = args.threads;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let subdomains = subdomain_enum::enumerate_subdomains(&target, wordlist.as_deref(), threads).await;
            let mut results = results.lock().unwrap();
            results.subdomains = subdomains;
        }));
    }
    
    if args.dns || args.all {
        let target = args.target.clone();
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let dns = dns_analysis::get_dns_records(&target).await;
            let mut results = results.lock().unwrap();
            results.dns_records = dns;
        }));
    }
    
    if run_tech {
        let target = args.target.clone();
        let timeout = args.timeout;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let tech = tech_detection::detect_technologies(&target, timeout).await;
            let mut results = results.lock().unwrap();
            results.technologies = tech;
        }));
    }
    
    if args.ssl || args.all {
        let target = args.target.clone();
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let ssl = ssl_analysis::analyze_ssl(&target).await;
            let mut results = results.lock().unwrap();
            results.ssl_info = ssl;
        }));
    }
    
    if args.waf || args.all {
        let target = args.target.clone();
        let timeout = args.timeout;
        let results = results.clone();
        
        tasks.push(tokio::spawn(async move {
            let waf = waf_detection::detect_waf(&target, timeout).await;
            let mut results = results.lock().unwrap();
            results.waf_detected = waf;
        }));
    }
    
    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
    
    // Enumerate subdomains with wordlist if requested
    if args.dns_wordlist.is_some() || args.all {
        let subdomains_from_dns = dns_analysis::enumerate_subdomains_with_wordlist(&args.target, args.dns_wordlist.as_deref()).await;
        let mut results = results.lock().unwrap();
        results.subdomains.extend(subdomains_from_dns);
    }
    
    // Get the final results
    let mut final_results = results.lock().unwrap();
    final_results.scan_duration = format!("{:?}", start.elapsed());
    
    // Output results
    match args.output.as_str() {
        "json" => output::print_json(&final_results)?,
        "html" => output::print_html(&final_results)?,
        _ => output::print_text(&final_results, args.verbose),
    }
    
    println!("\n✅ Scan completed in {:?}", start.elapsed());
    
    Ok(())
}

fn determine_modules(args: &Args) -> Vec<String> {
    let mut modules = Vec::new();
    
    if args.all || args.ports { modules.push("ports".to_string()); }
    if args.all || args.dirs { modules.push("dirs".to_string()); }
    if args.all || args.api { modules.push("api".to_string()); }
    if args.all || args.subdomains { modules.push("subdomains".to_string()); }
    if args.all || args.dns { modules.push("dns".to_string()); }
    if args.all || args.tech { modules.push("tech".to_string()); }
    if args.all || args.ssl { modules.push("ssl".to_string()); }
    if args.all || args.waf { modules.push("waf".to_string()); }
    
    // If no modules specified, run all
    if modules.is_empty() {
        modules = vec![
            "ports".to_string(), "dirs".to_string(), "api".to_string(), "subdomains".to_string(), 
            "dns".to_string(), "tech".to_string(), "ssl".to_string(), "waf".to_string()
        ];
    }
    
    modules
}
