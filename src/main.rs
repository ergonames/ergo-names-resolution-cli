use clap::{Arg, Command};
use ergonames::{resolve_ergoname, get_block_id_registered, get_block_registered, get_timestamp_registered, get_date_registerd, reverse_search};

fn main() {
    let cmds = Command::new("ergonames-cli")
        .version("0.0.1")
        .about("Resolves ErgoNames")
        .author("ErgoNames")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("resolve")
                .short_flag('r')
                .long_flag("resolve")
                .about("Resolves ErgoNames and addresses")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "blockid",
                            "blocknumber",
                            "timestamp",
                            "date",
                            "address"
                        ])
                )
                .arg(
                    Arg::new("blockid")
                        .short('i')
                        .long("blockid")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "name",
                            "blocknumber",
                            "timestamp",
                            "date",
                            "address"
                        ])
                )
                .arg(
                    Arg::new("blocknumber")
                        .short('N')
                        .long("blocknumber")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "name",
                            "blockid",
                            "timestamp",
                            "date",
                            "address",
                        ])
                )
                .arg(
                    Arg::new("timestamp")
                        .short('t')
                        .long("timestamp")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "name",
                            "blockid",
                            "blocknumber",
                            "date",
                            "address",
                        ])
                )
                .arg(
                    Arg::new("date")
                        .short('d')
                        .long("date")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "name",
                            "blockid",
                            "blocknumber",
                            "timestamp",
                            "address",
                        ])
                )
                .arg(
                    Arg::new("address")
                        .short('a')
                        .long("address")
                        .takes_value(true)
                        .conflicts_with_all(&[
                            "name",
                            "blockid",
                            "blocknumber",
                            "timestamp",
                            "date"
                        ])
                ),
        )
        .get_matches();


    match cmds.subcommand() {
        Some(("resolve", resolve_mathces)) => {
            let name: Option<&str> = resolve_mathces.value_of("name");
            let blockid: Option<&str> = resolve_mathces.value_of("blockid");
            let blocknumber: Option<&str> = resolve_mathces.value_of("blocknumber");
            let timestamp: Option<&str> = resolve_mathces.value_of("timestamp");
            let date: Option<&str> = resolve_mathces.value_of("date");
            let address: Option<&str> = resolve_mathces.value_of("address");
            if name.is_some() {
                let name: &str = name.unwrap();
                let reformated_name: String = format!("~{}", name);
                let name: &str = &reformated_name.as_str().to_owned();
                let resolved_ergoname: Option<String> = resolve_ergoname(name, None);
                if resolved_ergoname.is_none() {
                    println!("No ErgoName was found for {}", name);
                }
                else {
                    println!("{}", resolved_ergoname.unwrap());
                }
            }
            else if blockid.is_some() {
                let name: &str = blockid.unwrap();
                let reformated_name: String = format!("~{}", name);
                let name: &str = &reformated_name.as_str().to_owned();
                let blockid_registered: Option<String> = get_block_id_registered(name, None);
                if blockid_registered.is_none() {
                    println!("No ErgoName was found for {}", name);
                }
                else {
                    println!("{}", blockid_registered.unwrap());
                }
            }
            else if blocknumber.is_some() {
                let name: &str = blocknumber.unwrap();
                let reformated_name: String = format!("~{}", name);
                let blocknumber: &str = &reformated_name.as_str().to_owned();
                let blocknumber_registered: Option<i32> = get_block_registered(blocknumber, None);
                if blocknumber_registered.is_none() {
                    println!("No ErgoName was found for {}", blocknumber);
                }
                else {
                    println!("{}", blocknumber_registered.unwrap());
                }
            }
            else if timestamp.is_some() {
                let name: &str = timestamp.unwrap();
                let reformated_name: String = format!("~{}", name);
                let timestamp: &str = &reformated_name.as_str().to_owned();
                let timestamp_registered: Option<u64> = get_timestamp_registered(timestamp, None);
                if timestamp_registered.is_none() {
                    println!("No ErgoName was found for {}", timestamp);
                }
                else {
                    println!("{}", timestamp_registered.unwrap());
                }
            }
            else if date.is_some() {
                let name: &str = date.unwrap();
                let reformated_name: String = format!("~{}", name);
                let date: &str = &reformated_name.as_str().to_owned();
                let date_registered: Option<String> = get_date_registerd(date, None);
                if date_registered.is_none() {
                    println!("No ErgoName was found for {}", date);
                }
                else {
                    println!("{}", date_registered.unwrap());
                }
            }
            if address.is_some() {
                let address: &str = address.unwrap();
                let owned_tokens: Option<Vec<ergonames::Token>> = reverse_search(address, None);
                if owned_tokens.is_none() {
                    println!("No owned tokens found for {}", address);
                }
                else {
                    println!("{:?}", owned_tokens.unwrap());
                }
            }
        }
        _ => {
            println!("No valid subcommand provided");
        }
    }
}