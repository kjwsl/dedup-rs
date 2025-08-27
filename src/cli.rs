use clap_derive::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub path: String,
}
