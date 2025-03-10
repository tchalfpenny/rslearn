fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // will not compile if x is immutable
    println!("The value of x is: {x}");

    // const (always immutable), value *MUST* be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours");

    // shadowing examples
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}")
    }

    println!("The value of y is {y}");

    let spaces = "   ";
    println!("spaces string type: {spaces}.");
    let spaces = spaces.len();
    println!("spaces number type: {spaces}.");

    /*
        let mut spaces = "   ";
        spaces = spaces.len();

        this will cause a compile-time error,
        since you are not allowed to mutate a variable's type.
    */
}   