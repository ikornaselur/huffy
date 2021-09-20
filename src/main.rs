mod compress;
mod extract;
mod node;
use anyhow::Result;

use clap::{App, Arg};
use compress::compress;
use extract::extract;

fn main() -> Result<()> {
    let matches = App::new("Huffy")
        .version("0.1.0")
        .author("Axel <dev@absalon.is>")
        .about("Experimental Huddman coding for lossless compression")
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .help("Compress the input file")
                .required(true)
                .conflicts_with("extract"),
        )
        .arg(
            Arg::with_name("extract")
                .short("x")
                .long("extract")
                .help("Decompress the input file")
                .required(true)
                .conflicts_with("compress"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();

    if matches.is_present("compress") {
        compress(file_name)
    } else {
        extract(file_name)
    }
}
