use clap::{Command, Arg};

pub fn build_cli() -> Command{
    Command::new("xlit")
        .about("Xlit is a VCS built from void in Rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new xlit repository")
                .subcommand(
                    Command::new("hash-object")
                            .about("Store a file in the object database")
                            .arg(Arg::new("file")
                                .help("The file to hash")
                                .required(true)
                            )
                )   
        )
}