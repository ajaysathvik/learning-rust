fn main() {
    println!("Hello, world!");
    println!("I'm a AJAY");

    let my_num = 50;
    println!("My number is {}", my_num);

    let x = 5;
    let y = 10;
    println!("x = {} and y = {} and x+y ={}" , x, y,x+y);

    let z = x+y;
    println!("z = {}", z);

    let a = 5;
    let b = 10;
    let z = a+b;
    println!("a = {} and b = {} and a+b = {}", a,b,z);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered {}", input);
}
