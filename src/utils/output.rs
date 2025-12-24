use std::collections::HashMap;

pub fn print_json(result: &crate::ReconResult) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    println!("{}", json);
    Ok(())
}

pub fn print_html(result: &crate::ReconResult) -> anyhow::Result<()> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>IAMitul Report for {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #2c3e50; }}
        h2 {{ color: #3498db; }}
        table {{ border-collapse: collapse; width: 100%; margin-bottom: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
        .success {{ color: #27ae60; }}
        .warning {{ color: #f39c12; }}
        .danger {{ color: #e74c3c; }}
    </style>
</head>
<body>
    <h1>🔍 IAMitul Report</h1>
    <h2>Target: {}</h2>
    <p><strong>Scan Time:</strong> {}</p>
    <p><strong>Duration:</strong> {}</p>
    
    <h2>Open Ports</h2>
    <table>
        <tr><th>Port</th><th>Protocol</th><th>State</th><th>Service</th></tr>
        {}
    </table>
    
    <h2>Directories Found</h2>
    <ul>
        {}
    </ul>
    
    <h2>API Endpoints Found</h2>
    <ul>
        {}
    </ul>
    
    <h2>Subdomains Found</h2>
    <ul>
        {}
    </ul>
    
    <h2>DNS Records</h2>
    <table>
        <tr><th>Type</th><th>Records</th></tr>
        {}
    </table>
    
    <h2>Technologies Detected</h2>
    <ul>
        {}
    </ul>
    
    <h2>SSL Information</h2>
    {}
    
    <h2>WAF Detection</h2>
    {}
    
    <h2>Scan Options</h2>
    <p><strong>Threads:</strong> {}</p>
    <p><strong>Timeout:</strong> {}s</p>
    <p><strong>Modules:</strong> {}</p>
</body>
</html>"#,
        result.target,
        result.target,
        result.timestamp,
        result.scan_duration,
        format_ports_table(&result.ports),
        format_list(&result.directories),
        format_list(&result.api_endpoints),
        format_list(&result.subdomains),
        format_dns_table(&result.dns_records),
        format_list(&result.technologies),
        format_ssl_info(&result.ssl_info),
        format_waf_info(&result.waf_detected),
        result.scan_options.threads,
        result.scan_options.timeout,
        result.scan_options.modules_enabled.join(", ")
    );
    
    println!("{}", html);
    Ok(())
}

pub fn print_text(result: &crate::ReconResult, verbose: bool) {
    println!("\n🔍 IAMitul Results for {}", result.target);
    println!("📅 Scan Time: {}", result.timestamp);
    println!("⏱️ Duration: {}", result.scan_duration);
    
    println!("\n🚪 Open Ports:");
    for port in &result.ports {
        println!("  - {}/{} ({}) - {}", port.port, port.protocol, port.state, port.service);
    }
    
    println!("\n📁 Directories Found:");
    for dir in &result.directories {
        println!("  - {}", dir);
    }
    
    println!("\n🔌 API Endpoints Found:");
    for api in &result.api_endpoints {
        println!("  - {}", api);
    }
    
    println!("\n🌐 Subdomains Found:");
    for sub in &result.subdomains {
        println!("  - {}", sub);
    }
    
    println!("\n📋 DNS Records:");
    for (record_type, records) in &result.dns_records {
        if !records.is_empty() {
            println!("  {}: {}", record_type, records.join(", "));
        }
    }
    
    println!("\n🛠️ Technologies Detected:");
    for tech in &result.technologies {
        println!("  - {}", tech);
    }
    
    if let Some(ssl_info) = &result.ssl_info {
        println!("\n🔒 SSL Information:");
        println!("  Issuer: {}", ssl_info.issuer);
        println!("  Subject: {}", ssl_info.subject);
        println!("  Valid From: {}", ssl_info.valid_from);
        println!("  Valid To: {}", ssl_info.valid_to);
        println!("  Fingerprint: {}", ssl_info.fingerprint);
    }
    
    if let Some(waf) = &result.waf_detected {
        println!("\n🛡️ WAF Detected: {}", waf);
    }
    
    if verbose {
        println!("\n⚙️ Scan Options:");
        println!("  Threads: {}", result.scan_options.threads);
        println!("  Timeout: {}s", result.scan_options.timeout);
        println!("  Modules: {}", result.scan_options.modules_enabled.join(", "));
    }
}

fn format_ports_table(ports: &[crate::modules::port_scanner::PortResult]) -> String {
    ports.iter()
        .map(|p| format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>", 
            p.port, p.protocol, p.state, p.service))
        .collect::<String>()
}

fn format_list(items: &[String]) -> String {
    items.iter()
        .map(|i| format!("<li>{}</li>", i))
        .collect::<String>()
}

fn format_dns_table(dns: &HashMap<String, Vec<String>>) -> String {
    dns.iter()
        .map(|(t, r)| format!("<tr><td>{}</td><td>{}</td></tr>", t, r.join(", ")))
        .collect::<String>()
}

fn format_ssl_info(ssl: &Option<crate::modules::ssl_analysis::SslInfo>) -> String {
    match ssl {
        Some(info) => format!(
            "<table>
                <tr><td>Issuer</td><td>{}</td></tr>
                <tr><td>Subject</td><td>{}</td></tr>
                <tr><td>Valid From</td><td>{}</td></tr>
                <tr><td>Valid To</td><td>{}</td></tr>
                <tr><td>Fingerprint</td><td>{}</td></tr>
            </table>",
            info.issuer, info.subject, info.valid_from, info.valid_to, info.fingerprint
        ),
        None => "No SSL information available".to_string(),
    }
}

fn format_waf_info(waf: &Option<String>) -> String {
    match waf {
        Some(w) => format!("{} detected", w),
        None => "No WAF detected".to_string(),
    }
}
