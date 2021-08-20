
fn main() {

    // elements of a vector
    let array = [ 'a', 'b', 'c' ];
    for character in array {
        println!( "char is : {}", character );
    };

    // range >=2 and <5
    for number in (1..4).rev() {
        println!( "number is : {}", number );
    };
}

