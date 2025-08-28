use clap_derive::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub path: String,
    #[arg(short, long, default_value_t = false)]
    pub name_only: bool,
    #[arg(short, long, default_value_t = false)]
    pub no_numbering: bool,
}
