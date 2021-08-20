
fn main() {

    // elements of a vector
    let array = [ 'a', 'b', 'c' ];
    for character in array {
        println!( "char is : {}", character );
        // print : char is : a
        //         char is : b
        //         char is : c
    };

    // range >=2 and <5
    for number in (2..5).rev() {
        println!( "number is : {}", number );
        // print : number is : 4
        //         number is : 3
        //         number is : 2
    };
}

