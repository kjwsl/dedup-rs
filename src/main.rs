use clap::Parser;
use dedup_rs::{cli::Cli, find_dups};

fn main() {
    let cli = Cli::parse();

    let mut num = 0;
    match find_dups(&cli.path) {
        Ok(map) => {
            for (k, v) in map {
                if !cli.no_numbering {
                    print!("Dup #{}: ", num + 1);
                }
                println!("{}", k);

                if !cli.name_only {
                    for f in v {
                        println!("\t{}", f.display());
                    }
                }
                num += 1;
            }
        }
        Err(e) => println!("{}", e),
    }

    println!("Found {} duplicates", num);
}
