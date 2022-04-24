use crate::args::parse_args;
use crate::segment::segment;

mod args;
mod segment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    println!("Reading image");
    let img = image::open(args.img)?;
    println!("Processing image");
    let result = segment(&img, args.maxdiff);
    println!("Saving image");
    result.save(args.out)?;
    println!("Done!");
    Ok(())
}
