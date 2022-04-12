# eebus-rust

Playground project for checking out Rust and a very basic EEBUS implementation

## Introduction

This repository contains:

- Sample code to try out Rust, no proper design or whatsoever
- adoptions of the SPINE and SHIP EEBUS model definitions, they are likely issues and some models are not 100% correct
- (De-)serialization for EEBUS specific JSON format requirements
- Uses provided certificate, as the certificate generation code does not work yet
- Establishes a secure websocket connection and does a "golden path" SHIP client handshake
- Stops at the end, when the first SPINE request is received
- Browse the local network in an endless loop to find and connect to any EEBUS service

## Usage

```sh
Usage: cargo run <command>
Commands:
  browse
  connect <host> <port>
```

- `browse` will search mdns for EEBUS services and try to connect to them in an endless loop
- `connect` will connect to a specific EEBUS service by provide a host/ip adress and port address
