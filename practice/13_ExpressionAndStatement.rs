#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

fn main() {

    let a = 2 + 5;
    let b = ( 2 + 5 );
    let c = { 2 + 5 };
    let d = {
        2 + 5;
        3 + 6;
        10;
        7
    };

    println!( "a : {}, b : {}, c : {}, d : {}", a, b, c, d );
}

