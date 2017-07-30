extern crate cannon;

use cannon::*;

pub fn rect(console: &mut Console, x1: i16, y1: i16, x2: i16, y2: i16){
    let width = x2 - x1;
    let height = y2 - y1;

    for x in 0..width + 1 {
        for y in 0..height + 1{
            console.write_character(x +x1, y + y1, 32);
        }
    }
}
