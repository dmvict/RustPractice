
fn print_hello() {
    println!( "Hello, world!" );
}

fn four() -> i32 {
    4
}

fn increment( x: i32 ) -> i32 {
    x + 1
}

fn main() {

    print_hello();
    // print : Hello, world!

    let number = four();
    println!( "number : {}", number );
    // print : number : 4

    let incremented = increment( four() );
    println!( "incremented : {}", incremented );
    // print : incremented : 5
}

