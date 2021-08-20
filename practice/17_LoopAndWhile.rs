
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
        // print : counter is : 4
        //         counter is : 2
    };
    println!( "result is : {}", result );
    // print : result is : 10

    /* */

    let mut counter = 5;

    while counter != 0 {
        println!( "counter is : {}", counter );
        // print : counter is : 5
        //         counter is : 4
        //         counter is : 3
        //         counter is : 2
        //         counter is : 1
        counter -= 1;
    }
}

