[![CI](https://github.com/nwillc/rcryptoport/actions/workflows/CI.yml/badge.svg)](https://github.com/nwillc/rcryptoport/actions/workflows/CI.yml)
---
# rcryptoport

A command line crypto-currency portfolio status application implemented in Rust:

<pre>
$ rcryptoport<span style="color:red">
 BTC         0.103451  45997.35614482 (  -68.66)             4758.47 (   -7.10)
 ETH         7.030965   3418.74488592 (   -7.29)            24037.08 (  -51.27)</span><span style="color:green">
 SOL                     176.55593750 (    0.35)</span><span style="color:red">
 ZEC                     132.13343114 (   -0.42)
                                          Total:            28795.55 (  -58.37)</span>
</pre>

## Command Line Options
```shell
$ ./target/debug/rcryptoport -h
rcryptoport 2.0
nwillc@gmail.com
Retrieve current value of your crypto portfolio.

USAGE:
    rcryptoport [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -d, --dry-run    Dry run, do not save values
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Path to specific config file

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    setup    Set up portfolio configuration
```
## Configuration
Have your Nomics API key and holdings info ready. Setup your configuration:

```shell
$ rcryptoport setup
```

