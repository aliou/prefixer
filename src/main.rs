use std::env;
use std::io;
use std::io::BufRead;

// It seems like it's not possible to do something like:
//   const DEFAULT_PREFIX: &'static String = &String::from("#");
// See `rustc --explain E0015`.
const DEFAULT_PREFIX: &'static str = "#";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // So... `prefix` is not the same type depending on the branch it goes through,
    // but it works because I don't explicitly define the variable type.
    let prefix = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_PREFIX
    };

    // Get the standard input shared handle.
    let stdin = io::stdin();

    // Lock the handle so we can read from it.
    let handle = stdin.lock();

    // Unwrap the lines from the handle so we have an actual line and not a Result<line>.
    let unwrapped_lines = handle.lines().map(|line| line.unwrap());

    // Loop on the lines from the handle using the `BufRead` trait's `lines`.
    for line in unwrapped_lines {
        println!("{} {}", prefix, line)
    }

    Ok(())
}
