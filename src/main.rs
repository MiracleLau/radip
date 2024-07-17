#[macro_use] extern crate prettytable;
use local_ip_address::local_ip;
use prettytable::Table;
use reqwest;
use std::env;
use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 获取用户指定的接口地址，如果未提供则使用默认地址
    let binding = String::from("https://api.ipify.org");
    let url = args.get(1).unwrap_or(&binding);

    // 创建表格
    let mut table = Table::new();
    // 添加表头
    table.add_row(row![cb => "Area", "IP"]);

    // 获取局域网 IP
    match local_ip() {
        Ok(ip) => {
            table.add_row(row![c => "Local", ip.to_string()]);
        },
        Err(e) => eprintln!("Unable to get local IP address: {:?}", e),
    }

    // 获取外网 IP
    match get_public_ip(url).await {
        Ok(ip) => {
            table.add_row(row![c => "Public", ip.to_string()]);
        }
        Err(e) => eprintln!("Unable to get public IP address: {:?}", e),
    }
    table.printstd();
    Ok(())
}

async fn get_public_ip(url: &str) -> Result<IpAddr, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response.parse()?)
}