extern crate getopts;
use getopts::Options;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let mut opts = Options::new();

    opts.optflag("c", "", "The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.");
    opts.optflag("l", "", "The number of lines in each input file is written to the standard output.");
    opts.optflag("m", "", "The number of characters in each input file is written to the standard out- put.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.");

    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
    };

    let mut s = String::new();

    if matches.opt_present("c") || matches.opt_present("m") {
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => {
                if matches.opt_present("c") || matches.opt_present("m") {
                    println!("chars: {}", s.len())
                }

                if matches.opt_present("l") {
                    println!("lines: {}", s.lines().count())
                }

                println!("words: {}", s.split_whitespace().count())

            }
        }
    }
}
