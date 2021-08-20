
fn is_odd( x: i32 ) -> bool {
    if ( x & 1 ) == 0 {
        false
    } else {
        true
    }
}

fn tuple_from( x: i32, y: i32, z: u32 ) -> ( f32, u32 ) {
    let x = x as f32 + y as f32;
    let y = y as u32 + z;
    ( x, y )
}

fn factorial( x: u32 ) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => factorial( x - 1 ) * x,
    }
}

fn main() {

    println!( "7 is odd : {}, 10 is odd : {}", is_odd( 7 ), is_odd( 10 ) );

    println!( "tuple from numbers : {:?}", tuple_from( -2, 3, 10 ) );

    println!( "factorial of 5 : {:?}", factorial( 5 ) );
}

