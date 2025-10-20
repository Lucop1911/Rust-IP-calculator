use std::net::Ipv4Addr;
use std::str::FromStr;
use std::io::{self, Write};
use colored::*;

fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

fn u32_to_ipv4(num: u32) -> Ipv4Addr {
    Ipv4Addr::from(num.to_be_bytes())
}

fn subnet_mask(prefix: u8) -> Ipv4Addr {
    u32_to_ipv4((!0u32) << (32 - prefix))
}

fn calculate(ip: Ipv4Addr, prefix: u8) {
    let mask = subnet_mask(prefix);
    let ip_num = ipv4_to_u32(ip);
    let mask_num = ipv4_to_u32(mask);
    let network = ip_num & mask_num;
    let broadcast = network | !mask_num;

    println!("{}", "\nResults:".bright_cyan());
    println!("{} {}", "IP Address       :".bright_cyan(), ip.to_string().bright_green());
    println!("{} {}", "Subnet Mask      :".bright_cyan(), mask.to_string().bright_green());
    println!("{} {}", "Network Address  :".bright_cyan(), u32_to_ipv4(network).to_string().bright_green());
    println!("{} {}", "Broadcast Address:".bright_cyan(), u32_to_ipv4(broadcast).to_string().bright_green());

    if prefix < 31 {
        let first = u32_to_ipv4(network + 1);
        let last = u32_to_ipv4(broadcast - 1);
        println!("{} {} - {}", "Usable Range     :".bright_cyan(), first.to_string().bright_green(), last.to_string().bright_green());
    } else {
        println!("{}", "Usable Range     : N/A (No host addresses)".bright_red());
    }
    println!();
}

fn main() {
    loop {
        let mut input = String::new();
        print!("{}", "Enter IP/CIDR (e.g., 192.168.1.10/24) or 'exit':\n".blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting...".bright_yellow());
            break;
        }

        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            println!("{}", "Invalid format! Use 192.168.1.10/24\n".bright_red());
            continue;
        }
        
        let ip = match Ipv4Addr::from_str(parts[0]) {
            Ok(ip) => ip,
            Err(_) => {
                println!("{}", "Invalid IP address!\n".bright_red());
                continue;
            }
        };
        
        let prefix: u8 = match parts[1].parse() {
            Ok(p) if p <= 32 => p,
            _ => {
                println!("{}", "Invalid prefix! Use 0-32.\n".bright_red());
                continue;
            }
        };

        calculate(ip, prefix);
    }
}
