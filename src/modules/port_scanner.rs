use std::net::{SocketAddr, IpAddr, TcpStream as StdTcpStream};
use std::time::Duration;
use tokio::task;
use tokio::time::timeout;
use serde::Serialize;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Serialize, Clone)]
pub struct PortResult {
    pub port: u16,
    pub protocol: String,
    pub state: String,
    pub service: String,
    pub version: Option<String>,
}

pub async fn scan_ports(
    target: &str, 
    _threads: usize, 
    timeout_secs: u64
) -> Vec<PortResult> {
    let ports = vec![
        21, 22, 23, 25, 53, 80, 110, 111, 135, 139,
        143, 443, 993, 995, 1723, 3306, 3389, 5432,
        5900, 6379, 8080, 8443, 8888, 9200, 27017
    ];
    
    // First resolve the target to IP addresses
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(e) => {
            eprintln!("Failed to create DNS resolver: {}", e);
            return Vec::new();
        }
    };
    
    let ip_addrs = match resolver.lookup_ip(target).await {
        Ok(response) => response.iter().collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Failed to resolve {}: {}", target, e);
            return Vec::new();
        }
    };
    
    if ip_addrs.is_empty() {
        eprintln!("No IP addresses found for {}", target);
        return Vec::new();
    }
    
    let mut tasks = Vec::new();
    let timeout_duration = Duration::from_secs(timeout_secs);
    
    for port in ports {
        for ip_addr in &ip_addrs {
            let target = target.to_string();
            let ip_addr = *ip_addr;
            tasks.push(task::spawn(async move {
                scan_port(&target, ip_addr, port, timeout_duration).await
            }));
        }
    }
    
    let mut results = Vec::new();
    for task in tasks {
        if let Ok(Some(port_result)) = task.await {
            results.push(port_result);
        }
    }
    
    results
}

async fn scan_port(_target: &str, ip_addr: IpAddr, port: u16, timeout_duration: Duration) -> Option<PortResult> {
    let socket_addr = SocketAddr::new(ip_addr, port);
    
    // Use tokio::timeout to apply a timeout to the blocking connect operation
    let connect_future = async {
        StdTcpStream::connect_timeout(&socket_addr, timeout_duration)
    };
    
    match timeout(timeout_duration, connect_future).await {
        Ok(Ok(_stream)) => {
            let service = match port {
                21 => "FTP",
                22 => "SSH",
                23 => "Telnet",
                25 => "SMTP",
                53 => "DNS",
                80 => "HTTP",
                110 => "POP3",
                135 => "RPC",
                139 => "NetBIOS",
                143 => "IMAP",
                443 => "HTTPS",
                993 => "IMAPS",
                995 => "POP3S",
                1723 => "PPTP",
                3306 => "MySQL",
                3389 => "RDP",
                5432 => "PostgreSQL",
                5900 => "VNC",
                6379 => "Redis",
                8080 => "HTTP-Alt",
                8443 => "HTTPS-Alt",
                8888 => "HTTP-Alt",
                9200 => "Elasticsearch",
                27017 => "MongoDB",
                _ => "unknown",
            };
            
            Some(PortResult {
                port,
                protocol: "tcp".to_string(),
                state: "open".to_string(),
                service: service.to_string(),
                version: None,
            })
        }
        Ok(Err(_e)) => None,
        Err(_e) => None, // Timeout
    }
}
