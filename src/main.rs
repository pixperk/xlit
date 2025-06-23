use crate::cmd::{hash_object, init};

mod cmd;
mod cli;

fn main(){
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            match sub_m.subcommand() {
                Some(("hash-object", hash_m)) => {
                    let file = hash_m.get_one::<String>("file").unwrap();
                    hash_object::handle(file);
                },
                _ => {
                    init::handle();  // init without any sub-subcommand
                }
            }
        },
            
        _ => {
            eprintln!("Unknown command or subcommand");
        }
}
}