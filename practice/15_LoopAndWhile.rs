
fn main() {

    let mut counter = 5;

    let result = loop {
        if counter == 1 {
            break 10 * counter
        }
        counter -= 1;
        if counter == 3 || counter == 1 {
            continue
        }
        println!( "counter is : {}", counter );
    };
    println!( "result is : {}", result );

    /* */

    let mut counter = 5;

    while counter != 0 {
        println!( "counter is : {}", counter );
        counter -= 1;
    }
}

