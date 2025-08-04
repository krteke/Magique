// use crate::bimap::Bimap;
use crate::png::Png;
use crate::{chunk::Chunk, chunk_type::ChunkType};
use anyhow::anyhow;
use std::str::FromStr;
use std::{fs, path::PathBuf};

use crate::args::args;

const MAX_PNG_SIZE: u64 = i32::MAX as u64 + 1;

pub fn commands() -> Result<(), anyhow::Error> {
    let args_cmd = args();

    match args_cmd.subcommand() {
        Some(("encode", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();

            let bytes = read_file(path)?;

            let chunk_type = match_args.get_one::<String>("CHUNK_TYPE").unwrap();
            let message = match_args.get_one::<String>("MESSAGE").unwrap();
            let output_file = match_args.get_one::<PathBuf>("OUTPUT_FILE").unwrap();

            encode(bytes, chunk_type, message, output_file)?;
        }

        Some(("decode", match_args)) => {
            if let Some(s) = match_args.get_many::<String>("message") {
                let mut iter = s.collect::<Vec<&String>>().into_iter();

                let png_path = PathBuf::from(iter.next().unwrap());
                let chunk_type = iter.next().unwrap();
                let bytes = read_file(&png_path)?;

                decode(bytes, chunk_type)?;
            } else if let Some(s) = match_args.get_one::<PathBuf>("png") {
                let bytes = read_file(s)?;
                let chunk_type = "scPG";

                decode(bytes, chunk_type)?;
            }
        }

        Some(("remove", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();
            let bytes = read_file(path)?;

            let chunk_type = match_args.get_one::<String>("CHUNK_TYPE").unwrap();

            remove(bytes, chunk_type, path)?;
        }

        Some(("print", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();
            let bytes = read_file(path)?;

            print(bytes)?;
        }

        Some(("hide", match_args)) => {
            let path = match_args.get_one::<PathBuf>("OPNG_PATH").unwrap();
            let bytes = read_file(path)?;

            let hidden_files = match_args
                .get_many::<PathBuf>("HPNG_PATH")
                .unwrap()
                .collect::<Vec<&PathBuf>>();

            let output_file = match_args.get_one::<PathBuf>("OUTPUT_FILE").unwrap();
            hide(bytes, hidden_files, output_file)?;
        }

        _ => {}
    }

    Ok(())
}

fn encode(
    bytes: Vec<u8>,
    chunk_type: &str,
    message: &str,
    output_file: &PathBuf,
) -> Result<(), anyhow::Error> {
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    let mut png = Png::try_from(bytes.as_slice())?;
    png.append_chunk(chunk);
    let png_bytes = png.as_bytes();

    match fs::write(output_file, png_bytes) {
        Ok(_) => {
            println!("成功将秘密消息封印到了 {:?} 文件里", output_file);
        }
        Err(e) => {
            return Err(anyhow!("无法写入文件到 {:?} ，错误是: {}", output_file, e));
        }
    }

    Ok(())
}

fn decode(bytes: Vec<u8>, chunk_type: &str) -> Result<(), anyhow::Error> {
    let png = Png::try_from(bytes.as_slice())?;
    if chunk_type != "scPG" {
        let message = png.search_chunk(chunk_type)?;

        println!("解密后的消息是：{}", message);
    } else {
        let mut num = 0;

        for chunk in png.chunks() {
            if *chunk.chunk_type() == ChunkType::from_str("scPG")? {
                let output_path = PathBuf::from(format!("./{}.png", num));

                match fs::write(&output_path, chunk.data()) {
                    Ok(_) => {
                        println!("成功解密 {:?}", output_path);
                    }
                    Err(e) => {
                        return Err(anyhow!("无法写入文件到 {:?} ，错误是: {}", output_path, e));
                    }
                }
                num += 1;
            }
        }
    }

    Ok(())
}

fn remove(bytes: Vec<u8>, chunk_type: &str, path: &PathBuf) -> Result<(), anyhow::Error> {
    let mut png = Png::try_from(bytes.as_slice())?;

    png.remove_first_chunk(chunk_type)?;
    let png_bytes = png.as_bytes();

    match fs::write(path, png_bytes) {
        Ok(_) => {
            println!("成功移除了{:?}文件中的秘密消息", path);
        }
        Err(e) => {
            return Err(anyhow!("无法写入文件到 {:?} ，错误是: {}", path, e));
        }
    }

    Ok(())
}

fn print(bytes: Vec<u8>) -> Result<(), anyhow::Error> {
    let png = Png::try_from(bytes.as_slice())?;
    println!("{}", png);

    Ok(())
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => return Err(anyhow!("Failed to read PNG file: {}", e)),
    };

    Result::Ok(bytes)
}

fn hide(
    bytes: Vec<u8>,
    hidden_files: Vec<&PathBuf>,
    output_file: &PathBuf,
) -> Result<(), anyhow::Error> {
    let mut original_png = Png::try_from(bytes.as_slice())?;

    // let mut a = Bimap::new();
    // a.insert(b"IHDR", b"scHD")
    //     .insert(b"PLTE", b"scPL")
    //     .insert(b"IDAT", b"scID")
    //     .insert(b"IEND", b"scEN")
    //     .insert(b"tRNS", b"scTR")
    //     .insert(b"gAMA", b"scGA")
    //     .insert(b"cHRM", b"scCH")
    //     .insert(b"sRGB", b"scSR")
    //     .insert(b"iCCP", b"scIC")
    //     .insert(b"iTXt", b"scTX")
    //     .insert(b"tEXt", b"scEX")
    //     .insert(b"zTXt", b"scZT")
    //     .insert(b"bKGD", b"scBK")
    //     .insert(b"pHYs", b"scPH")
    //     .insert(b"sBIT", b"scBT")
    //     .insert(b"sPLT", b"scSP");

    for file in hidden_files {
        if std::fs::metadata(file)?.len() > MAX_PNG_SIZE {
            return Err(anyhow!("File {:?} is too large to hide", file));
        } else {
            let data = read_file(file)?;
            let chunk_type = ChunkType::from_str("scPG")?;
            let chunk = Chunk::new(chunk_type, data);

            original_png.append_chunk(chunk);
        }
    }

    let png_bytes = original_png.as_bytes();

    match fs::write(output_file, png_bytes) {
        Ok(_) => println!("成功将文件隐藏在 {:?} 中", output_file),
        Err(e) => return Err(anyhow!("Failed to write PNG file: {}", e)),
    }

    Ok(())
}
