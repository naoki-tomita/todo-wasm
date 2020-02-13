#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

use crate::rest::routes::start_server;

mod domains;
mod drivers;
mod error;
mod gateways;
mod ports;
mod rest;
mod usecases;

#[macro_use]
extern crate rocket;

fn main() {
    start_server()
}
