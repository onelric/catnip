extern crate ansi_term;
extern crate getopts;
extern crate os_info;
extern crate systemstat;

use std::env;

use ansi_term::{ANSIString, Color};
use getopts::Options;
use util::*;

mod util;

fn main() {
    // Define args
    let mut opts = Options::new();

    opts.optopt("f", "file", "loads an ascii file from path", "FILE");
    opts.optopt(
        "p",
        "padding",
        "sets padding between ascii and data",
        "PADDING",
    );
    opts.optopt(
        "s",
        "seperator",
        "chnages the symbol between the ascii and statistics",
        "SEPERATOR",
    );

    // Get args
    let args: Vec<String> = env::args().collect();
    let file_path: FetchResult<String> = fetch_argument(&args, &opts, "f");
    let padding: usize = fetch_argument(&args, &opts, "p").unwrap_or(3);
    let sep: String = fetch_argument(&args, &opts, "s").unwrap_or("".to_owned());

    // Retrieve data
    let ascii = file_path.map_or(get_ascii(), |path| {
        load_ascii(path.as_str()).unwrap_or(get_ascii())
    });
    let mem = get_memory();
    let distro = get_distro();

    let d = format!("{} {} {}", distro[0], sep, distro[1]);
    let w = format!("{} {} {}", "󰖲", sep, get_window_manager());
    let p = format!("{} {} {}", "󰏖", sep, get_packages());
    let m = format!("{} {} {} / {}", "󰍛", sep, mem[0], mem[1]);

    // Create data array in order
    let data = vec![d.as_str(), w.as_str(), p.as_str(), m.as_str()];
    let mut fetch = String::new();

    // Get longest line in the ascii to properly space the information
    let longest = ascii.iter().map(|x| x.len()).max().unwrap();

    for i in 0..ascii.len() {
        let mut ascii_line = ascii[i].to_owned();

        if i < data.len() {
            // Add spacing elements
            for _ in 0..longest + padding as usize - ascii_line.len() {
                ascii_line.push(' ');
            }

            // Render data with color
            let d = data[i];
            let text = match i {
                0 => Color::Blue.italic().paint(d),
                1 => Color::Yellow.italic().paint(d),
                2 => Color::Green.italic().paint(d),
                3 => Color::Purple.italic().paint(d),
                _ => ANSIString::from(""),
            };

            fetch += format!(" {}{}\n", ascii_line, text).as_str();
        }
    }

    println!("\n{}\n", fetch)
}
