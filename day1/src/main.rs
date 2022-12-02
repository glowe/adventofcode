#![feature(binary_heap_into_iter_sorted)]
use std::io;
use std::collections::BinaryHeap;


/// .
///
/// # Errors
///
/// This function will return an error if .
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut heap = BinaryHeap::new();
    let mut calories = 0;
    loop {
        let mut buf = String::new();
        let n = stdin.read_line(&mut buf)?;
        match n {
            0 => break,
            1 => {
                heap.push(calories);
                calories = 0;
                continue;
            }
            _ => calories += buf.trim_end().parse::<u32>().unwrap(),
        }
    }
    let total = heap.into_iter_sorted().take(3).fold(0, |acc, x| acc + x);
    println!("{}", total);
    Ok(())
}
