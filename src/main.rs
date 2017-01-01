#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate docopt;
extern crate rustc_serialize;

mod point;
mod ray;
mod shapes;
mod scene;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use docopt::Docopt;

use scene::Scene;
use shapes::Sphere;
use point::Point;

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

fn load_scene(filename: &String) {
    let path = Path::new(filename);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file
    };

    match serde_json::from_reader(file) {
        Err(why) => panic!("Couldn't load {}: {:?}", path.display(), why),
        Ok(scene) => scene
    }
}

fn serialize_scene() {
    let sphere = Sphere { origin: Point { x: 1.0, y: 1.0, z: 1.0 }, radius: 1.0 };
    let objects = vec![sphere];
    let scene: Scene = Scene { objects: objects };
    println!("{}", serde_json::to_string(&scene).unwrap());
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_debug {
        println!("Debug mode enabled");
    }

    let scene = load_scene(&args.arg_SCENE);
    println!("{:?}", scene);
//    serialize_scene();
}
