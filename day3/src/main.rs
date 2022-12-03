use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn priority(item: &char) -> Option<u32> {
    if *item >= 'A' && *item <= 'Z' {
        return Some(*item as u32 - 38);
    }
    if *item >= 'a' && *item <= 'z' {
        return Some(*item as u32 - 96);
    }
    None
}

const GROUP_SIZE: u8 = 3;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut priorities = 0;
    'out: loop {
        let mut buf = String::new();

        for _ in 0..GROUP_SIZE {
            let n = stdin.read_line(&mut buf)?;
            if n == 0 {
                break 'out;
            }
        }
        let mut frequency = HashMap::new();

        for rucksack in buf.split_whitespace() {
            let mut set = HashSet::new();
            for item in rucksack.chars() {
                set.insert(item);
            }
            for item in set.into_iter() {
                let freq = frequency.entry(item).or_insert(0);
                *freq += 1;
                if *freq == GROUP_SIZE {
                    priorities += priority(&item).unwrap();
                }
            }
        }
    }
    println!("{}", priorities);
    Ok(())
}
