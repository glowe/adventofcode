use std::collections::HashMap;
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
    let mut seen = HashMap::new();

    // Use a hashmap histogram windowed to the length of the marker and if its length
    // matches the length of the marker then we know the characters are unique. Relying
    // on HashMap::len() to be O(1) yields an overall runtime of O(n)
    for (i, ch) in datastream.char_indices() {
        if i >= marker_size {
            // Decrement an occurence of the character left adjecent to the left side of the window
            let c = datastream.chars().nth(i - marker_size).unwrap();
            let val = seen.get_mut(&c).unwrap();
            *val -= 1;
            // ...and if occurences is 0, then remove the entry from the hashmap, so that its length
            // will correspond to the number of unique characters seen in the window
            if *val == 0 {
                seen.remove(&c);
            }
        }
        let entry = seen.entry(ch).or_insert(0);
        *entry += 1;
        if seen.len() == marker_size {
            println!("{}", i + 1);
            break;
        }
    }
    Ok(())
}
