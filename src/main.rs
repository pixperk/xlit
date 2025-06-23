mod cli;
mod cmd;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            cmd::init::handle();
        },
        Some(("hash-object", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            cmd::hash_object::handle(file);
        },
        Some(("cat-file", sub_m)) => {
            let hash = sub_m.get_one::<String>("hash").unwrap();
            cmd::cat_file::handle(hash);
        },
        _ => unreachable!(),
    }
}
