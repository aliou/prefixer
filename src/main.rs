use std::env;
use std::io;
use std::io::BufRead;

// It seems like it's not possible to do something like:
//   const DEFAULT_PREFIX_CHARACTER: &'static String = &String::from("#");
// See `rustc --explain E0015`.
const DEFAULT_PREFIX_CHARACTER: &'static str = "#";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // So... `character` is not the same type depending on the branch it goes through,
    // but it works because I don't explicitly define the character type.
    let character = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_PREFIX_CHARACTER
    };

    // Get the standard input shared handle.
    let stdin = io::stdin();

    // Lock the handle so we can read from it.
    let handle = stdin.lock();

    // Loop on the lines from the handle using the `BufRead` trait's `lines`.
    for line in handle.lines() {
        match line {
            Ok(string) => println!("{} {}", character, string),
            _ => (),
        }
    }

    Ok(())
}
