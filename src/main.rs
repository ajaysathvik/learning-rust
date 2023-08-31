// use std::io::stdin;
// use std::f64;
// use rand::Rng;

fn main() {
    // println!("Hello, world!");
    // println!("I'm a AJAY");

    // let my_num = 50;
    // println!("My number is {}", my_num);

    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {} and x+y ={}" , x, y,x+y);

    // let z = x+y;
    // println!("z = {}", z);

    // let mut a = 5;
    // let b = 10;
    // let z = a+b;
    // println!("a = {} and b = {} and a+b = {}", a,b,z);

    // let mut input = String::new(); // mutable variable
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // print!("You entered {}", input);

    // println!("{}",a*b);
    // println!("{}",a/b);
    // println!("{}",a%b);
    // println!("{}",a-b);

    // a+=1;
    // println!("{}",a);

    // let mut name = String::new();
    // std::io::stdin().read_line(&mut name).expect("Failed to read line");
    // let greetings = String::from("Hello");
    // name.replace_range(0..1, ""); // replace the first character of the string with empty string
    // name = name.trim().to_string(); // trim() is used to remove the white spaces
    // println!("{} {}",greetings,name);
    // println!("{}",greetings.len());
    // println!("{}",name.len());

    // let mut x = String::new();
    // stdin().read_line(&mut x).expect("Failed to read line");
    // let _x: usize = x.trim().parse().expect("Failed to parse x"); // parse() is used to convert string to integer
    // let mut y = String::new();
    // stdin().read_line(&mut y).expect("Failed to read line");
    // let _y:usize=y.trim().parse().expect("failed to parse y");
    // if _x>_y {
    //     println!("x is greater than y");
    // }
    // else if _x<_y {
    //     println!("x is less than y");
    // }
    // else {
    //     println!("x is equal to y");
    // }

    // let x: f64 = 16.0;
    // println!("{}",x.sqrt());

    // let mut x = String::new();
    // stdin().read_line(&mut x).expect("Failed to read line");
    // let _x:usize = x.trim().parse().expect("Failed to parse x");
    // let _y = (_x as f64).sqrt();
    // println!("{}",_y);

    // let is_coding_fun = true;
    // let is_fish_tasty = true;
    // println!("{}",is_coding_fun);
    // println!("{}",is_fish_tasty);
    // let x = 5;
    // let y = 10;
    // println!("{}",x>y);

    // let x = 5;
    // let y = 10;
    // if x > y {
    //     println!("x is greater than y");
    // }
    // else if y>x{
    //     println!("x is less than y");
    // }
    // else {
    //     println!("x is equal to y");
    // }

    // let arr = [0,1,2,3,4,5];
    // for i in arr.iter() {
    //     print!("{} ",i);
    // }

    // let secret_number = rand::thread_rng().gen_range(1..=100); // generate random number between 1 and 100
    // // println!("The secret number is: {}", secret_number);
    // loop {
    //     println!("Guess the number!");
    //     let mut guess = String::new();
    //     stdin().read_line(&mut guess).expect("Failed to read line");
    //     let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //     println!("You guessed: {}", guess);
    //     if guess == secret_number {
    //         println!("You win!");
    //         break;
    //     } else {
    //         println!("You lose!");
    //     }
    // }

    /* DAY7 */
    // let mut a=0;
    // loop {
    //     println!("{}",a);
    //     a+=1;
    // } // infinite loop

    // let mut a = 0;
    // while a < 10 {
    //     println!("{}", a);
    //     a += 1;
    // } // while loop

    // for i in 0..10 {
    //     println!("{}", i);
    // }// for loop with range 0 to 9

    let arr = [1,2,3,4,5];
    for a in 0..arr.len() {
        println!("{}", arr[a]); // print array elements
    }
    for a in arr.iter() {
        println!("{}", a); // print array elements
    }
    for a in arr{
        println!("{}", a); // print array elements
    }
    for a in arr.iter().rev() {
        println!("{}", a); // print array elements in reverse order
    }
    for a in arr.len()-1..0 {
        println!("{}", arr[a]); // print array elements in reverse order
    }
    for a in (0..arr.len()).rev() {
        println!("{}", arr[a]); // print array elements in reverse order
    }

    
}
