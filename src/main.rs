use std::net::Ipv4Addr;
use std::str::FromStr;
use std::io::{self, Write};
use colored::*;

fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    let octets = ip.octets();
    ((octets[0] as u32) << 24)
        | ((octets[1] as u32) << 16)
        | ((octets[2] as u32) << 8)
        | (octets[3] as u32)
}

fn u32_to_ipv4(num: u32) -> Ipv4Addr {
    Ipv4Addr::new(
        ((num >> 24) & 0xFF) as u8,
        ((num >> 16) & 0xFF) as u8,
        ((num >> 8) & 0xFF) as u8,
        (num & 0xFF) as u8,
    )
}

fn subnet_mask(prefix: u8) -> Ipv4Addr {
    let mask: u32 = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };
    u32_to_ipv4(mask)
}

fn main() {
    loop {
        let mut input = String::new();
        print!("{}", "Enter IP/CIDR (e.g., 192.168.1.10/24) or 'exit' to quit:\n".blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting...".bright_yellow());
            break;
        }

        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            println!("{}", "Invalid format! Use IP/CIDR like 192.168.1.10/24\n".bright_red());
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
                println!("{}", "Invalid prefix! Use a number between 0 and 32.\n".bright_red());
                continue;
            }
        };

        let mask = subnet_mask(prefix);
        let ip_num = ipv4_to_u32(ip);
        let mask_num = ipv4_to_u32(mask);

        let network = ip_num & mask_num;
        let broadcast = network | !mask_num;

        let first_host = if prefix < 31 { network + 1 } else { network };
        let last_host = if prefix < 31 { broadcast - 1 } else { broadcast };

        let str_ip = ip.to_string();
        let str_mask = mask.to_string();
        let str_network = u32_to_ipv4(network).to_string();
        let str_broadcast = u32_to_ipv4(broadcast).to_string();
        let str_first_host = u32_to_ipv4(first_host).to_string();
        let str_last_host = u32_to_ipv4(last_host).to_string();
        
        println!("{}", "\nResults:".bright_cyan());
        println!("{} {}", "IP Address       :".bright_cyan(), str_ip.bright_green());
        println!("{} {}", "Subnet Mask      :".bright_cyan(), str_mask.bright_green());
        println!("{} {}", "Network Address  :".bright_cyan(), str_network.bright_green());
        println!("{} {}", "Broadcast Address:".bright_cyan(), str_broadcast.bright_green());
        println!("{} {} - {}\n", "Usable Range     :".bright_cyan(), str_first_host.bright_green(), str_last_host.bright_green());
    }
}
