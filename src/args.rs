use clap::Parser;

/// Image segmenter
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the input image
    #[clap(short, long)]
    pub img: String,

    /// Path to the output image
    #[clap(short, long)]
    pub out: String,

    /// (0-1) Maximum difference between two colors to consider the same color
    #[clap(short, long, default_value_t=0.05)]
    pub maxdiff: f32
}

pub fn parse_args() -> Args {
    Args::parse()
}
