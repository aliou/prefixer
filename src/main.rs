use std::io;
use std::io::BufRead;

const DEFAULT_PREFIX_CHARACTER: &'static str = "#";

fn main() -> io::Result<()> {
    // Get the standard input shared handle.
    let stdin = io::stdin();

    // Lock the handle so we can read from it.
    let handle = stdin.lock();

    // Loop on the lines from the handle using the `BufRead` trait's `lines`.
    for line in handle.lines() {
        match line {
            Ok(string) => println!("{}  {}", DEFAULT_PREFIX_CHARACTER, string),
            _ => (),
        }
    }

    Ok(())
}
