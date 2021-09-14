[![CI](https://github.com/nwillc/rcryptoport/actions/workflows/CI.yml/badge.svg)](https://github.com/nwillc/rcryptoport/actions/workflows/CI.yml)
---
# rcryptoport

A command line crypto-currency portfolio status application implemented in Rust. Uses [Nomics](https://p.nomics.com/cryptocurrency-bitcoin-api) free API.

```shell
$ rcryptoport
Symbol     Price       Change         Holding             Position   Change
 BTC  46694.39368896 (   -6.76)       0.103451             4830.58 (   -0.70)
 ETH   3362.54223815 (    1.67)       7.030965            23641.92 (   11.71)
 SOL    153.38500094 (    0.23)
 ZEC    130.81561919 (   -0.14)
                                       Total:             28472.50 (   11.01)
``

## Command Line Options
```shell
$ rcryptoport -h
rcryptoport 2.0.0
nwillc@gmail.com
Retrieve current value of your crypto portfolio.

USAGE:
    rcryptoport [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -d, --dry-run    Dry run, do not save values
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>     Path to specific config file
    -l, --loop <SECONDS>    Run looping every SECONDS seconds

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    setup    Set up portfolio configuration
```
## Configuration
Have your Nomics API key and holdings info ready. Setup your configuration:

```shell
$ rcryptoport setup
```

## TODO

- Add timestamp to config and note time elapsed since last run in output.
