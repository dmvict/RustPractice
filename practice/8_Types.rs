
fn main() {

    // signed integers : i8, i16, i32, i64, i128, isize
    let signed: i16 = -2;
    println!( "signed is : {}", signed );

    // unsugned untegers : u8, u16, u32, u64, u128, usuze
    let unsigned: u64 = 2;
    println!( "unsigned is : {}", unsigned );

    // floating point : f32, f64
    let floating_default = 2.3215320;
    println!( "floating_default is : {}", floating_default );
    let floating: f32 = 2.3215320;
    println!( "floating is : {}", floating );

    // characters and booleans
    let c = 'c';
    let d = 'D';
    let is_chars = true;
    println!( "{} and {} is chars :  {}", c, d, is_chars );
}

