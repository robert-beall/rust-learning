fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let x: &str = "5";
    let x: u32 = x.parse().expect("OOPS!");

    println!("{x}");
}