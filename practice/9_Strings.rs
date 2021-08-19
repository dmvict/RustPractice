
fn main() {

    // static string literal
    let slice = "Hello, Learn-Together-Pro!";
    println!( "length : {}, is empty : {}", slice.len(), slice.is_empty() );
    println!( "slice as bytes : {:?}", slice.as_bytes() );

    // string
    let mut string = String::from( "Hello" );
    string.push( ' ' );
    string.push_str( "Learn-Together-Pro!" );
    string.insert( 5, ',' );
    println!( "string :\n{}", string );
}

