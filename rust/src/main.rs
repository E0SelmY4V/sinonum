use std::io;

use sinonum::*;

pub fn main() {
    let line = {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("cannot read!");
        str
    };
    print!("{}", sinonumify(&line, LiangOption::WithUnit));
}
