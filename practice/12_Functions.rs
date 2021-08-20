
fn print( slice: &str ) {
    println!( "{}", slice );
}

fn four() -> i32 {
    4
}

fn increment( x: i32 ) -> i32 {
    x + 1
}

fn main() {

    let hello = "Hello, world!";
    print( hello );

    let number = four();
    println!( "number : {}", number );

    let incremented = increment( four() );
    println!( "incremented : {}", incremented );
}

