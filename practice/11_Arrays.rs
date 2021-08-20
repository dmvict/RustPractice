
fn main() {

    let a = [ 1.1, 2.2, 3.3 ];
    let mut b: [ u16; 3 ] = [ 1, 2, 3 ];
    let c = [ 0; 5 ];
    println!( "a : {:?}, b : {:?}, c : {:?}", a, b, c );

    let first = a[ 0 ];
    println!( "first : {:?}", first );

    println!( "b : {:?}", b );
    b[ 2 ] = 5;
    println!( "b : {:?}", b );
}

