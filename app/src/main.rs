use std::process;

use structopt::StructOpt;

struct Point<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    /*   let opt = lsclone::Opt::from_args();
    if let Err(ref e) = lsclone::run(&opt.path, opt.show_hidden_files) {
        println!("{}", e);
        process::exit(1);
    } */

    let x = 3;
    let r;
    {
        let y = 4;
        let point = Point { x: &x, y: &y };
        r = point.x
    }
    println!("{}", r);
}
