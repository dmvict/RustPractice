
fn is_odd( x: i32 ) -> bool {
    if ( x & 1 ) == 0 {
        false
    } else {
        true
    }
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
    // print : 7 is odd : true, 10 is odd : false

    println!( "factorial of 5 : {:?}", factorial( 5 ) );
    // print : factorial of 5 : 120
}

