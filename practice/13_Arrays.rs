
fn main() {

    let a = [ 1.1, 2.2, 3.3 ];
    let mut b: [ u16; 3 ] = [ 1, 2, 3 ];
    let c = [ 0; 5 ];
    println!( "a : {:?}, b : {:?}, c : {:?}", a, b, c );
    // print : a : [1.1, 2.2, 3.3], b : [1, 2, 3], c : [0, 0, 0, 0, 0]

    let first = a[ 0 ];
    println!( "first : {}", first );
    // print :first : 1.1

    println!( "b : {:?}", b );
    // print : b : [1, 2, 3]
    b[ 2 ] = 5;
    println!( "b : {:?}", b );
    // print : b : [1, 2, 5]
}

