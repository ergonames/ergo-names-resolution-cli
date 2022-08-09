# ErgoNames Resolution CLI


```
ergo-names-resolution-cli 0.0.2
ErgoNames
ergo-names-resolution-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help                    Print this message or the help of the given subcommand(s)
    resolve -r --resolve    Resolves ErgoNames and addresses

    SUBCOMMANDS:
        name -n --name          Name of the ErgoNames to resolve

            ARGUMENTS:
                help                            Print this message or the help of the given argument(s)
                owner -o --owner                Resolves the owner's address of the ErgoName
                blockid -i --blockid            Resolves the block id that the ErgoName was registered in
                blocknumber -n --blocknumber    Resolves the block number that the ErgoName was registered in
                timestamp -t --timestamp        Resolves the timestamp that the ErgoName was registered at
                date -d --date                  Resolves the date that the ErgoName was registered on

        address -a --address    Address of the ErgoNames to resolve

            ARGUMENTS:
                help                            Print this message or the help of the given argument(s)
                owned -O --owned                Gets a list of ErgoNames owned by the address
                amount -a --amount              Gets the total amount of ErgoNames owned by the address

```