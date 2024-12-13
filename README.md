# PPS Time
A Rust interface for Pulse Per Second (PPS) devices, based on [RFC 2783](https://datatracker.ietf.org/doc/html/rfc2783). This crate is part of [Project Pendulum](https://github.com/pendulum-project), and is used in [ntpd-rs](https://github.com/pendulum-project/ntpd-rs).

The `pps.rs` contains bindings that were automatically generated by rust-bindgen:
```
bindgen /usr/include/linux/pps.h -o ./src/pps.rs --with-derive-default --raw-line '#![allow(dead_code, non_camel_case_types)]'
```
