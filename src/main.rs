use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "split-tilemap")]
struct Cli {
    width: i32,
    height: i32,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
