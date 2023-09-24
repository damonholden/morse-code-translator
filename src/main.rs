use clap::Parser;

#[derive(Parser)]
struct Cli {
    morse_code: String,
}

fn main() {
    let args = Cli::parse();

    print!("Morse code: {}", args.morse_code)
}
