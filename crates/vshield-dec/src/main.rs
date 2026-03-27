use clap::Parser;
use std::path::PathBuf;
use vshield_dec::{Decoder, DecoderConfig};

#[derive(Parser, Debug)]
#[command(name = "V-Shield Decoder")]
#[command(about = "Extract hidden files from YouTube-resistant video frames", long_about = None)]
struct Args {
    /// Input directory containing frame PNG files
    #[arg(short, long)]
    input: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Decryption token
    #[arg(short, long)]
    token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("╔════════════════════════════════════╗");
    println!("║       V-Shield Decoder CLI         ║");
    println!("║     v0.1.0 - Phase 1 Alpha        ║");
    println!("╚════════════════════════════════════╝\n");

    // Validate inputs
    if !args.input.exists() {
        return Err(format!("Input directory not found: {:?}", args.input).into());
    }

    let config = DecoderConfig {
        input_frames_dir: args.input.to_string_lossy().to_string(),
        output_file: args.output.to_string_lossy().to_string(),
        token: args.token.clone(),
    };

    let decoder = Decoder::new(config);

    println!("📝 Configuration:");
    println!("   Input:  {:?}", args.input);
    println!("   Output: {:?}", args.output);
    let token = &args.token;
    let preview_start = &token[..token.len().min(16)];
    let preview_end = &token[token.len().saturating_sub(8)..];
    println!("   Token:  {}...{}", preview_start, preview_end);
    println!();

    println!("🔧 Decoding...");
    let output = decoder.decode()?;

    println!("\n✅ Decoding complete!");
    println!("   Original file:  {}", output.metadata.filename);
    println!("   Size:           {} bytes", output.metadata.file_size);
    println!("   Hash verified:  ✓");
    println!();

    println!("💾 Saving output...");
    output.save(&args.output.to_string_lossy())?;

    println!("\n🎉 Success! Your file has been extracted.");
    println!("   File saved to: {:?}", args.output);

    Ok(())
}
