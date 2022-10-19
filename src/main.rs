use clap::Parser;
use flat_chart::{run, Args};

fn main() {
    let args = Args::parse();

    run(args);
}
