use clap::{ArgMatches, Command, arg, command, value_parser};
use std::path::PathBuf;

pub fn args() -> ArgMatches {
    let matches = command!()
        .subcommand(
            Command::new("encode")
                .about("encode a png")
                .arg(
                    arg!(<PNG_PATH> "require a path to a png file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(<CHUNK_TYPE> "require a chunk type"))
                .arg(arg!(<MESSAGE> "require a message"))
                .arg(
                    arg!([OUTPUT_FILE] "require a path to output file")
                        .required(false)
                        .default_value("./output.png")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("decode")
                .about("decode a png")
                .arg(
                    arg!(<PNG_PATH> "require a path to a png file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(<CHUNK_TYPE> "require a chunk type")),
        )
        .subcommand(
            Command::new("remove")
                .about("remove a message")
                .arg(
                    arg!(<PNG_PATH> "require a path to a png file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(<CHUNK_TYPE> "require a chunk type")),
        )
        .subcommand(Command::new("print").about("print a png").arg(
            arg!(<PNG_PATH> "require a path to a png file").value_parser(value_parser!(PathBuf)),
        ))
        .get_matches();

    matches
}
