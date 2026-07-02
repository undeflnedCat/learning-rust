// this program will not compile! if not for these tweaks

fn main() {
    // WRONG! compile error
    // let x = 55;

    // correct!
    let mut x = 5;

    println!("The value of x is {x}.");
    x = 6;
    println!("The value of x is {x}.")
}
