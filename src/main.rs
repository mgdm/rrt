extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const USAGE: &'static str = "
Usage: rrt [-d] SCENE
       rrt (--help | --version)

Options:
    -d, --debug     Enable debug output
    -h, --help      Show this message
    --version       Show the version
";

#[derive(RustcDecodable)]
struct Args {
    arg_SCENE: String,
    flag_debug: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_debug {
        println!("Debug mode enabled");
    }
}
