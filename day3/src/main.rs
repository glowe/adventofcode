use std::collections::HashSet;
use std::io;

fn priority(ch: &char) -> Option<u32> {
    if *ch >= 'A' && *ch <= 'Z' {
        return Some(*ch as u32 - 38);
    }
    if *ch >= 'a' && *ch <= 'z' {
        return Some(*ch as u32 - 96);
    }
    None
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut priorities = 0;
    loop {
        let mut buf = String::new();
        let n = stdin.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        let mut set = HashSet::new();
        let buf = buf.trim_end();
        let mid = buf.len() / 2;
        buf.chars().take(mid).for_each(|c| {
            set.insert(c);
        });
        buf.chars().skip(mid).for_each(|c| {
            if set.contains(&c) {
                let p = priority(&c).unwrap();
                priorities += p;
                // Prevent double counting by removing char
                set.remove(&c);
            }
        });
    }
    println!("{}", priorities);
    Ok(())
}

// adding priority 16 for p
// adding priority 38 for L
// adding priority 38 for L
// adding priority 42 for P
// adding priority 22 for v
// adding priority 22 for v
// adding priority 20 for t
// adding priority 20 for t
// adding priority 19 for s
