use std::process;
use colored::Colorize;

use structopt::StructOpt;

struct Point<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    let opt = lsclone::Opt::from_args();
    if let Err(ref e) = lsclone::run(&opt.path, opt.show_hidden_files) {
        println!("{}", e);
        process::exit(1);
    }

    println!(
        "{}, {}, {}, {}, {}, {}, and some normal text.",
        format!("Bold").bold(),
        format!("Red").red(),
        format!("Yellow").yellow(),
        format!("Green Strikethrough").green().strikethrough(),
        format!("Blue Underline").blue().underline(),
        format!("Purple Italics").purple().italic()
    );

    /*     let x = 3;
    let r;
    {
        let y = 4;
        let point = Point { x: &x, y: &y };
        r = point.x
    }
    println!("{}", r); */
}
