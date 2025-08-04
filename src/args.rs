use clap::{ArgAction, ArgGroup, ArgMatches, Command, arg, command, value_parser};
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
                    arg!(--message <PNG_PATH> "Decode a secret message from a PNG file")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(2)
                        .value_names(["PNG_PATH", "CHUNK_TYPE"]),
                )
                .arg(
                    arg!(--png <PNG_PATH> "Decode a hidden PNG from a PNG file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .group(
                    ArgGroup::new("vars")
                        .required(true)
                        .args(["message", "png"]),
                ),
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
        .subcommand(
            Command::new("hide")
                .about("hide png(s) into another png")
                .arg(
                    arg!(<OPNG_PATH> "require a path to a original png file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(<HPNG_PATH> "file to be hidden")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Append),
                )
                .arg(
                    arg!([OUTPUT_FILE] "require a path to output file")
                        .required(false)
                        .default_value("./output.png")
                        .value_parser(value_parser!(PathBuf))
                        .last(true),
                ),
        )
        .get_matches();

    matches
}
