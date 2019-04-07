use std::io;
use std::io::prelude::*;

#[macro_use]
extern crate failure;

use failure::Error;

extern crate pisa_lib;
use pisa_lib::Deck;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "passphrase", short = "p")]
    passphrase: String,

    #[structopt(long = "keycard1", requires = "keycard2" short = "c", help = "specify which keycards (number between 0 and 53 inclusive) to use, if left empty then the first 2 letters of the passphrase will be used")]
    keycard1: Option<u8>,

    #[structopt(
        long = "keycard2",
        short = "C",
        help = "specify which keycards (number between 0 and 53 inclusive) to use, if left empty then the first 2 letters of the passphrase will be used"
    )]
    keycard2: Option<u8>,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() {
    let stdin = io::stdin();
}
