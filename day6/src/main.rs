use std::collections::HashSet;
use std::env;
use std::error;
use std::io;
use std::process;
use std::result;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("usage: {} <marker_size>", args.nth(0).unwrap());
        process::exit(1);
    }
    let marker_size = args.nth(1).unwrap().parse::<usize>()?;
    let stdin = io::stdin();
    let mut datastream = String::new();
    stdin.read_line(&mut datastream)?;
    let datastream = datastream.trim_end();
    'out: for i in 0..datastream.len() - marker_size {
        let mut set = HashSet::new();
        for c in datastream[i..i + marker_size].chars() {
            if set.contains(&c) {
                continue 'out;
            }
            set.insert(c);
        }
        println!("{}", i + marker_size);
        break;
    }
    Ok(())
}
