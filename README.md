# Definition Import Tool (DIT)
DIT is a tool that allows Hyperview users to import and maintain BACnet and Modbus **(Roadmap)** definitions from the command line. Hyperview has a GUI for users to manage BACnet and Modbus definitions, however, in larger sites with hundreds or thousands of sensors, a command line tool makes it easier to manage the definitions in bulk. 

# Configuration
A valid Hyperview API client must be used. The API client must have the appropriate access. The configuration file must be placed in `$HOME/.hyperview/hyperview.toml`

## Example

```console
client_id = 'c33472d0-c66b-4659-a8f8-73c289ba4dbe'
client_secret = '2c239e21-f81b-472b-a8c3-82296d5f250d'
scope = 'HyperviewManagerApi'
auth_url = 'https://example.hyperviewhq.com/connect/authorize'
token_url = 'https://example.hyperviewhq.com/connect/token'
instance_url = 'https://example.hyperviewhq.com'
```

# Usage
DIT has various commands for the various actions it can perform. 

```console
$ ./dit help                                                                                                                  main 
An import tool for BACnet and Modbus sensor definitions for Hyperview

Usage: dit [OPTIONS] <COMMAND>

Commands:
  get-bacnet-definitions          List current BACnet definitions
  add-bacnet-definition           Add a new BACnet definition
  get-bacnet-numeric-sensors      Get a list of existing numeric sensors for a specific definition
  get-bacnet-non-numeric-sensors  Get a list of existing non-numeric sensors for a specific definition
  add-bacnet-numeric-sensor       Adds numeric sensors to a definition
  add-bacnet-non-numeric-sensor   Adds non-numeric sensors to a definition
  get-sensor-types                Get sensor types compatible with an asset type
  help                            Print this message or the help of the given subcommand(s)

Options:
  -l, --debug-level <DEBUG_LEVEL>  Debug level [default: error] [possible values: trace, debug, info, warn, error]
  -h, --help                       Print help
  -V, --version                    Print version
```

# Building

## Linux
If you are experimenting with the code on a single platform the usual `cargo build` and `cargo build --release` will work. However, if the desire is to build a binary that can run on multiple Linux distributions it is recommended to install the `x86_64-unknown-linux-musl` target and to build a statically-linked binary to avoid dependency problems. 

The command to build a statically-linked version is:

```console
PKG_CONFIG_SYSROOT_DIR=/ RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-unknown-linux-musl --release
```

## Windows & MacOS
The usual `cargo build` and `cargo build --release` will work. 
