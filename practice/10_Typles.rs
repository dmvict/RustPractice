
fn main() {

    let tup_explicit: ( i32, &str ) = ( -20, "str" );
    let tup_implicit = ( 2.0, 10, -20 );
    println!( "tup_explicit : {:?}, tup_implicit : {:?}", tup_explicit, tup_implicit );

    let ( x, y, z ) = tup_implicit;
    println!( "x : {}, y : {}, z : {}", x, y, z );

    let x = tup_implicit.0;
    let y = tup_implicit.1;
    let z = tup_implicit.2;
    println!( "x : {}, y : {}, z : {}", x, y, z );
}

