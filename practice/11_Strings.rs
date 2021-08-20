
fn main() {

    // static string literal
    let slice = "Hello, Learn-Together-Pro!";
    println!( "length : {}, is empty : {}", slice.len(), slice.is_empty() );
    // print : length : 26, is empty : false
    println!( "slice as bytes : {:?}", slice.as_bytes() );
    // print : slice as bytes : [72, 101, 108, 108, 111, 44, 32, 76, 101, 97, 114, 110, 45, 84, 111, 103, 101, 116, 104, 101, 114, 45, 80, 114, 111, 33]

    // string
    let mut string = String::from( "Hello" );
    string.push( ' ' );
    string.push_str( "Learn-Together-Pro!" );
    string.insert( 5, ',' );
    println!( "string :\n{}", string );
    // print : string :
    //         Hello, Learn-Together-Pro!
}

