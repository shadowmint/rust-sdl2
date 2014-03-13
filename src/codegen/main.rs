#[feature(macro_rules)];
#[crate_id = "codegen#0.1"];

use std::os;
use std::io::BufferedWriter;
use std::io::File;
use std::io;
use std::io::stdio::println;
use std::io::fs::mkdir_recursive;
use std::path::GenericPath;
pub mod branchify;
pub mod keycode;
pub mod scancode;

fn main() {
    let args = os::args();
    match args.len() {
        0 => {
            println("usage: codegen [keycode|scancode].rs destdir");
            os::set_exit_status(1);
        },
        3 => {
            let output_dir = GenericPath::new(args[2].clone());
            // TODO: maybe not 0777?
            mkdir_recursive(&output_dir, 0b111_111_111);
            if (args[1] == ~"keycode.rs") {
                keycode::generate(&output_dir);
            }
            else if (args[1] == ~"scancode.rs") {
                scancode::generate(&output_dir);
            }
        },
        _ => {
            println!("usage: {} [keycode|scancode].rs destdir", args[0]);
            os::set_exit_status(1);
        }
    }
}

pub fn get_writer(output_dir: &Path, filename: &str) -> ~BufferedWriter<File> {
    match File::open_mode(&output_dir.join(filename), io::Truncate, io::Write) {
        Ok(writer) => ~BufferedWriter::new(writer),
        Err(e) => fail!("Unable to write file: {:s}", e.desc),
    }
}
