use anyhow::Error;
use clap::Parser;
use iso2god::converter::{Cli, convert_iso_to_god};
fn main() -> Result<(), Error> {
    let args = Cli::parse();
    convert_iso_to_god(args)
}
