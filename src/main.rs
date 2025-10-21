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

fn subnet_mask(prefix: u8) -> u32 {
    if prefix == 0 { 0 } else { (!0u32) << (32 - prefix) }
}

fn calculate_prefix_for_hosts(hosts: u32) -> u8 {
    let total_needed = hosts + 2;
    let bits_needed = (total_needed as f64).log2().ceil() as u8;
    32 - bits_needed
}

#[derive(Debug)]
struct SubnetInfo {
    subnet_number: usize,
    required_hosts: u32,
    network: Ipv4Addr,
    prefix: u8,
    mask: Ipv4Addr,
    first_usable: Ipv4Addr,
    last_usable: Ipv4Addr,
    broadcast: Ipv4Addr,
    total_hosts: u32,
}

fn calculate_subnets(base_ip: Ipv4Addr, base_prefix: u8, host_counts: Vec<u32>) -> Result<Vec<SubnetInfo>, String> {
    let base_mask_num = subnet_mask(base_prefix);
    let base_network_num = ipv4_to_u32(base_ip) & base_mask_num;
    let base_broadcast = base_network_num | !base_mask_num;

    let mut sorted: Vec<(usize, u32)> = host_counts.iter().enumerate().map(|(i, &h)| (i, h)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut subnets = Vec::new();
    let mut current_network = base_network_num;

    for (original_index, hosts) in sorted {
        let prefix = calculate_prefix_for_hosts(hosts);
        if prefix < base_prefix {
            return Err(format!("Subnet {} needs /{} but base is /{}", original_index + 1, prefix, base_prefix));
        }

        let mask_num = subnet_mask(prefix);
        let network = current_network & mask_num;
        let broadcast = network | !mask_num;

        if broadcast > base_broadcast {
            return Err(format!("Subnet {} does not fit in base network", original_index + 1));
        }

        let total_hosts = (broadcast - network + 1) - 2;
        let first_usable = u32_to_ipv4(network + 1);
        let last_usable = u32_to_ipv4(broadcast - 1);

        subnets.push((original_index, SubnetInfo {
            subnet_number: original_index + 1,
            required_hosts: hosts,
            network: u32_to_ipv4(network),
            prefix,
            mask: u32_to_ipv4(mask_num),
            first_usable,
            last_usable,
            broadcast: u32_to_ipv4(broadcast),
            total_hosts,
        }));

        current_network = broadcast + 1;
    }

    subnets.sort_by_key(|(_, info)| ipv4_to_u32(info.network));
    Ok(subnets.into_iter().map(|(_, info)| info).collect())
}

fn display_subnets(base_ip: Ipv4Addr, base_prefix: u8, subnets: Vec<SubnetInfo>) {
    println!("\n{}", "═══════════════════════════════════════════════════════════".bright_cyan());
    println!("{} {}/{}", "Base Network:".bright_cyan().bold(), base_ip.to_string().bright_green(), base_prefix.to_string().bright_green());
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan());

    for subnet in subnets {
        println!("\n{} {}", "Subnet".bright_yellow().bold(), subnet.subnet_number.to_string().bright_yellow().bold());
        println!("{}", "───────────────────────────────────────────────────────────".bright_black());
        println!("{:<20} {}", "Required Hosts:".bright_cyan(), subnet.required_hosts.to_string().bright_white());
        println!("{:<20} {}", "Available Hosts:".bright_cyan(), subnet.total_hosts.to_string().bright_white());
        println!("{:<20} {}", "Network Address:".bright_cyan(), subnet.network.to_string().bright_green());
        println!("{:<20} {}", "Subnet Mask:".bright_cyan(), subnet.mask.to_string().bright_green());
        println!("{:<20} /{}", "CIDR Notation:".bright_cyan(), subnet.prefix.to_string().bright_green());
        println!("{:<20} {}", "First Usable IP:".bright_cyan(), subnet.first_usable.to_string().bright_magenta());
        println!("{:<20} {}", "Last Usable IP:".bright_cyan(), subnet.last_usable.to_string().bright_magenta());
        println!("{:<20} {}", "Broadcast Address:".bright_cyan(), subnet.broadcast.to_string().bright_red());
    }

    println!("\n{}", "═══════════════════════════════════════════════════════════".bright_cyan());
}

fn main() {
    println!("{}", "╔═══════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                   Subnet Calculator                   ║".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".bright_cyan());
    
    loop {
        println!("\n{}", "Enter base network (IP/CIDR, e.g., 192.168.1.0/24) or 'exit':".blue());
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting...".bright_yellow());
            break;
        }
        
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            println!("{}", "Invalid format! Use xxx.xxx.xxx.xxx/yy\n".bright_red());
            continue;
        }
        
        let base_ip = match Ipv4Addr::from_str(parts[0]) {
            Ok(ip) => ip,
            Err(_) => {
                println!("{}", "Invalid IP address!\n".bright_red());
                continue;
            }
        };
        
        let base_prefix: u8 = match parts[1].parse() {
            Ok(p) if p <= 32 => p,
            _ => {
                println!("{}", "Invalid prefix! Use 0-32.\n".bright_red());
                continue;
            }
        };
        
        println!("\n{}", "Enter number of subnets to create:".blue());
        let mut num_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut num_input).expect("Failed to read input");
        
        let num_subnets: usize = match num_input.trim().parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("{}", "Invalid number of subnets!\n".bright_red());
                continue;
            }
        };
        
        let mut host_counts = Vec::new();
        for i in 1..=num_subnets {
            println!("\n{} {}:", "Enter required hosts for subnet".blue(), i.to_string().bright_yellow());
            let mut hosts_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut hosts_input).expect("Failed to read input");
            
            let hosts: u32 = match hosts_input.trim().parse() {
                Ok(h) if h > 0 => h,
                _ => {
                    println!("{}", "Invalid host count!\n".bright_red());
                    continue;
                }
            };
            host_counts.push(hosts);
        }
        
        match calculate_subnets(base_ip, base_prefix, host_counts) {
            Ok(subnets) => return display_subnets(base_ip, base_prefix, subnets),
            Err(e) => println!("{} {}\n", "Error:".bright_red().bold(), e.bright_red()),
        }
    }
}