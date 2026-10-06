use clap::{Parser, ValueEnum};
use std::path::{PathBuf, Path};
use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::HashMap;

const BUF_SIZE: usize = 64 * 1024;

#[derive(Parser)]
struct Args {
    /// File to inspect
    #[arg(short, long)]
    file: PathBuf,

    /// Symbol unit: b = bit, B = byte
    #[arg(short, long, value_enum)]
    symbol: SymbolSize,
}

#[derive(Clone, Copy, ValueEnum)]
enum SymbolSize {
    #[value(name = "b")]
    Bit,

    #[value(name = "B")]
    Byte
}

fn compute_entropy(path: &Path) -> std::io::Result<f64> {
    let reader = BufReader::with_capacity(BUF_SIZE, File::open(path)?);

    let mut symbols_freq: HashMap<u8, usize> = HashMap::new();

    // Entropy, as I understand it, is the weighted average of the bits
    // necessary to encode each symbol. Weighted by how often each symbol
    // appears.
    for byte in reader.bytes() {
        *symbols_freq.entry(byte?).or_insert(0) += 1;
    }

    let total: usize = symbols_freq.values().sum();

    let mut entropy: f64 = 0.0;

    for (_, symbol_freq) in symbols_freq {
        let symbol_p: f64 = symbol_freq as f64 / total as f64;
        entropy += -symbol_p * symbol_p.log2();
    }

    Ok(entropy)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let ent = compute_entropy(&args.file);

    println!("Entropy {}", ent?);

    Ok(())
}
