use anyhow::Result;
use mp_stats_converter::Converter;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Usage: converter [input_dir] [output_dir]
    let input_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("data")
    };

    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("target/converted_data")
    };

    let converter = Converter::new(input_dir, output_dir)?;
    converter.convert()?;

    Ok(())
}
