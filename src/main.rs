use clap::{Arg, Command};
use ergonames::{resolve_ergoname, get_block_id_registered, get_block_registered, get_timestamp_registered, get_date_registerd, reverse_search, get_total_amount_owned, Token};

fn main() {
    let cmds = Command::new("ergo-names-resolution-cli")
        .version("0.0.2")
        .about("Resolves ErgoNames Information")
        .author("ErgoNames")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("resolve")
                .short_flag('r')
                .long_flag("resolve")
                .about("Resolves ErgoNames and addresses")
                .subcommand(
                    Command::new("name")
                        .short_flag('n')
                        .long_flag("name")
                        .about("Resolves ErgoNames")
                        .arg(Arg::with_name("owner")
                            .short('o')
                            .long("owner")
                            .takes_value(true)
                            .help("Get the owner address of the ErgoName")
                        )
                        .arg(Arg::with_name("blockid")
                            .short('i')
                            .long("blockid")
                            .takes_value(true)
                            .help("Get the block id that the ErgoName was registered at")
                        )
                        .arg(Arg::with_name("blocknumber")
                            .short('n')
                            .long("blocknumber")
                            .takes_value(true)
                            .help("Get the block number that the ErgoName was registered at")
                        )
                        .arg(Arg::with_name("timestamp")
                            .short('t')
                            .long("timestamp")
                            .takes_value(true)
                            .help("Get the timestamp that the ErgoName was registered at")
                        )
                        .arg(Arg::with_name("date")
                            .short('d')
                            .long("date")
                            .takes_value(true)
                            .help("Get the date that the ErgoName was registered at")
                        )
                    )
                    .subcommand(
                        Command::new("address")
                            .short_flag('a')
                            .long_flag("address")
                            .about("Resolves addresses")
                            .arg(Arg::with_name("owned")
                                .short('O')
                                .long("owned")
                                .takes_value(true)
                                .help("Get all the ErgoNames that are owned by the address")
                            )
                            .arg(Arg::with_name("amount")
                                .short('a')
                                .long("amount")
                                .takes_value(true)
                                .help("Get the amount of ErgoNames that are registered at the address")
                            )
                    )

                )
                .get_matches();


    match cmds.subcommand() {
        Some(("resolve", subcmds)) => {
            match subcmds.subcommand() {
                Some(("name", name_subcmds)) => {
                    let owner: Option<&str> = name_subcmds.value_of("owner");
                    let blockid: Option<&str> = name_subcmds.value_of("blockid");
                    let blocknumber: Option<&str> = name_subcmds.value_of("blocknumber");
                    let timestamp: Option<&str> = name_subcmds.value_of("timestamp");
                    let date: Option<&str> = name_subcmds.value_of("date");
                    if owner.is_some() {
                        let owner: &str = owner.unwrap();
                        let reformated: String = format!("~{}", owner);
                        let owner: &str = reformated.as_str();
                        let resolved: String = resolve_ergoname(owner, None).unwrap();
                        println!("{}", resolved);
                    } else if blockid.is_some() {
                        let blockid: &str = blockid.unwrap();
                        let reformated: String = format!("~{}", blockid);
                        let blockid: &str = reformated.as_str();
                        let resolved: String = get_block_id_registered(blockid, None).unwrap();
                        println!("{}", resolved);
                    } else if blocknumber.is_some() {
                        let blocknumber: &str = blocknumber.unwrap();
                        let reformated: String = format!("~{}", blocknumber);
                        let blocknumber: &str = reformated.as_str();
                        let resolved: i32 = get_block_registered(blocknumber, None).unwrap();
                        println!("{}", resolved);
                    } else if timestamp.is_some() {
                        let timestamp: &str = timestamp.unwrap();
                        let reformated: String = format!("~{}", timestamp);
                        let timestamp: &str = reformated.as_str();
                        let resolved: u64 = get_timestamp_registered(timestamp, None).unwrap();
                        println!("{}", resolved);
                    } else if date.is_some() {
                        let date: &str = date.unwrap();
                        let reformated: String = format!("~{}", date);
                        let date: &str = reformated.as_str();
                        let resolved: String = get_date_registerd(date, None).unwrap();
                        println!("{}", resolved);
                    }
                }
                Some(("address", address_subcmds)) => {
                    let address: Option<&str> = address_subcmds.value_of("owned");
                    let amount: Option<&str> = address_subcmds.value_of("amount");
                    if address.is_some() {
                        let address: &str = address.unwrap();
                        let resolved: Vec<Token> = reverse_search(address, None).unwrap();
                        println!("{:?}", resolved);
                    }
                    if amount.is_some() {
                        let amount: &str = amount.unwrap();
                        let resolved: u32 = get_total_amount_owned(amount, None).unwrap();
                        println!("{}", resolved);
                    }
                }
                _ => {
                    println!("No subcommand given");
                }
            }
        }
        _ => {
            println!("No command given");
        }
    }
}