extern crate clap;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate tokio_core;

use clap::App;
use clap::Arg;
use futures::{Future, Stream};
use hyper::{Client, Uri};
use std::io::{self, Write};
use tokio_core::reactor::Core;

fn send(req: String) {
    let uri = req.parse::<Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::new();

    let work = client.get(uri).map(|res| {
        println!("Response: {:?}", res);
    });
    let resp = core.run(work).unwrap();

    println!("{:?}", resp);
}

fn main() {
    let matches = App::new("pong")
        .version("0.1.0")
        .author("Josh Brudnak <jobrud314@gmail.com>")
        .about("A for pinging a url")
        .arg(
            Arg::with_name("INPUT")
                .value_name("URL")
                .required(true)
                .help("The URL to use")
                .index(1),
        ).get_matches();

    let url = matches
        .value_of("INPUT")
        .expect("Url not chosen")
        .to_string();

    let response = send(url);
}
