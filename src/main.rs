use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let b = byte.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("{:?} is a goofy character\r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
    
        if c == 'q' {
            break;
        }
    }
}
