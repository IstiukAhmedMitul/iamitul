# IAMitul 🚀  
**High-Performance Reconnaissance Tool**

IAMitul is a blazing fast, comprehensive reconnaissance tool written in **Rust** for security professionals and penetration testers.

---

## ✨ Features

- ⚡ **High Performance** – Built with Rust for maximum speed and efficiency  
- 🔍 **Comprehensive Scanning** – All-in-one reconnaissance capabilities  
- 🧩 **Modular Design** – Enable only the modules you need  
- 📊 **Multiple Output Formats** – Text, JSON, and HTML reports  
- 🛡️ **WAF Detection** – Identify Web Application Firewalls  
- 🔒 **SSL Analysis** – Detailed certificate information  
- 🌐 **Subdomain Enumeration** – Discover related domains  
- 📁 **Directory Brute-Forcing** – Find hidden directories and files  
- 🔌 **API Discovery** – Uncover API endpoints  
- 🚪 **Port Scanning** – Identify open ports and services  
- 📋 **DNS Analysis** – Gather DNS records  
- 🛠️ **Technology Detection** – Identify web technologies  

---

## 📦 Installation

### From Source (Recommended)

#### 1️⃣ Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2️⃣ Clone and Build
git clone https://github.com/yourusername/iamitul.git
cd iamitul
cargo build --release

3️⃣ Install System-Wide
cargo install --path .

🚀 Usage
Basic Scan
iamitul example.com

Comprehensive Scan
iamitul example.com --all --threads 20 --timeout 15

JSON Output
iamitul example.com --all --output json > scan_results.json

HTML Report
iamitul example.com --all --output html > report.html

Specific Modules
iamitul example.com --ports --ssl --waf --verbose

Custom Wordlist
iamitul example.com --dirs --wordlist /path/to/wordlist.txt

🧾 Command Line Options
IAMitul 0.1.0
High-performance reconnaissance tool

USAGE:
    iamitul [OPTIONS] <TARGET>

ARGS:
    <TARGET>    Target domain or IP

OPTIONS:
    -a, --all                 Enable all modules
        --api                 Enable API discovery
        --dns                 Enable DNS analysis
        --dirs                Enable directory brute-forcing
    -h, --help                Print help information
        --ports               Enable port scanning
        --ssl                 Enable SSL analysis
        --subdomains          Enable subdomain enumeration
        --tech                Enable technology detection
        --threads <THREADS>   Threads to use (default: auto)
    -t, --timeout <TIMEOUT>   Timeout in seconds (default: 10)
    -v, --verbose             Enable verbose output
        --waf                 Enable WAF detection
    -w, --wordlist <WORDLIST> Custom wordlist path
    -o, --output <OUTPUT>     Output format (text, json, html)
    -V, --version             Print version information

🧩 Modules
🔌 Port Scanner

Scans common TCP ports

Identifies service and version

Fast asynchronous scanning

📁 Directory Brute-Forcing

Discovers hidden directories and files

Supports custom and built-in wordlists

Concurrent requests for speed

🔍 API Discovery

Finds common API endpoints

Identifies REST and GraphQL APIs

Supports custom endpoint lists

🌐 Subdomain Enumeration

Discovers subdomains using multiple techniques

Supports custom wordlists

Concurrent DNS resolution

📋 DNS Analysis

Gathers DNS records (A, AAAA, MX, NS, TXT, SOA)

Identifies potential misconfigurations

🛠️ Technology Detection

Identifies frameworks, servers, and libraries

Analyzes headers and content

🔒 SSL Analysis

Detailed certificate information

Identifies SSL/TLS versions

Checks for common vulnerabilities

🛡️ WAF Detection

Identifies common WAF products

Analyzes headers and responses
