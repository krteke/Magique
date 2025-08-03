use crate::png::Png;
use crate::{chunk::Chunk, chunk_type::ChunkType};
use anyhow::anyhow;
use std::str::FromStr;
use std::{fs, path::PathBuf};

use crate::args::args;

pub fn commands() -> Result<(), anyhow::Error> {
    let args_cmd = args();

    match args_cmd.subcommand() {
        Some(("encode", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();

            let bytes = read_file(path)?;

            let chunk_type = match_args.get_one::<String>("CHUNK_TYPE").unwrap();
            let message = match_args.get_one::<String>("MESSAGE").unwrap();
            let output_file = match_args.get_one::<PathBuf>("OUTPUT_FILE").unwrap();

            let _ = encode(bytes, chunk_type, message, output_file)?;
        }

        Some(("decode", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();
            let bytes = read_file(path)?;

            let chunk_type = match_args.get_one::<String>("CHUNK_TYPE").unwrap();

            let _ = decode(bytes, chunk_type)?;
        }

        Some(("remove", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();
            let bytes = read_file(path)?;

            let chunk_type = match_args.get_one::<String>("CHUNK_TYPE").unwrap();

            let _ = remove(bytes, chunk_type, path)?;
        }

        Some(("print", match_args)) => {
            let path = match_args.get_one::<PathBuf>("PNG_PATH").unwrap();
            let bytes = read_file(path)?;

            let _ = print(bytes)?;
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
    let message = png.search_chunk(chunk_type)?;

    println!("解密后的消息是：{}", message);

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
