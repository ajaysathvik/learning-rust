// use std::io::stdin;
use std::collections::HashMap;
// use std::f64;
// use rand::Rng;

use std::process::Stdio;

fn main() {
    /* Day 6 */
    // println!("Hello, world!");
    // println!("I'm a AJAY");

    // let my_num = 50;
    // println!("My number is {}", my_num);

    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {} and x+y ={}", x, y, x + y);

    // let z = x + y;
    // println!("z = {}", z);

    // let mut a = 5;
    // let b = 10;
    // let z = a + b;
    // println!("a = {} and b = {} and a+b = {}", a, b, z);

    // let mut input = String::new(); // mutable variable
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
    // print!("You entered {}", input);

    // println!("{}", a * b);
    // println!("{}", a / b);
    // println!("{}", a % b);
    // println!("{}", a - b);

    // a += 1;
    // println!("{}", a);

    // let mut name = String::new();
    // std::io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    // let greetings = String::from("Hello");
    // name.replace_range(0..1, ""); // replace the first character of the string with empty string
    // name = name.trim().to_string(); // trim() is used to remove the white spaces
    // println!("{} {}", greetings, name);
    // println!("{}", greetings.len());
    // println!("{}", name.len());

    // let mut x = String::new();
    // stdin().read_line(&mut x).expect("Failed to read line");
    // let _x: usize = x.trim().parse().expect("Failed to parse x"); // parse() is used to convert string to integer
    // let mut y = String::new();
    // stdin().read_line(&mut y).expect("Failed to read line");
    // let _y: usize = y.trim().parse().expect("failed to parse y");
    // if _x > _y {
    //     println!("x is greater than y");
    // } else if _x < _y {
    //     println!("x is less than y");
    // } else {
    //     println!("x is equal to y");
    // }

    // let x: f64 = 16.0;
    // println!("{}", x.sqrt());

    // let mut x = String::new();
    // stdin().read_line(&mut x).expect("Failed to read line");
    // let _x: usize = x.trim().parse().expect("Failed to parse x");
    // let _y = (_x as f64).sqrt();
    // println!("{}", _y);

    // let is_coding_fun = true;
    // let is_fish_tasty = true;
    // println!("{}", is_coding_fun);
    // println!("{}", is_fish_tasty);
    // let x = 5;
    // let y = 10;
    // println!("{}", x > y);

    // let x = 5;
    // let y = 10;
    // if x > y {
    //     println!("x is greater than y");
    // } else if y > x {
    //     println!("x is less than y");
    // } else {
    //     println!("x is equal to y");
    // }

    // let arr = [0, 1, 2, 3, 4, 5];
    // for i in arr.iter() {
    //     print!("{} ", i);
    // }

    // let secret_number = rand::thread_rng().gen_range(1..=100); // generate random number between 1 and 100
    //                                                            // println!("The secret number is: {}", secret_number);
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
    // let mut a = 0;
    // loop {
    //     println!("{}", a);
    //     a += 1;
    // } // infinite loop

    // let mut a = 0;
    // while a < 10 {
    //     println!("{}", a);
    //     a += 1;
    // } // while loop

    // for i in 0..10 {
    //     println!("{}", i);
    // } // for loop with range 0 to 9

    // let arr = [1, 2, 3, 4, 5];
    // for a in 0..arr.len() {
    //     println!("{}", arr[a]); // print array elements
    // }
    // for a in arr.iter() {
    //     println!("{}", a); // print array elements
    // }
    // for a in arr {
    //     println!("{}", a); // print array elements
    // }
    // for a in arr.iter().rev() {
    //     println!("{}", a); // print array elements in reverse order
    // }
    // for a in arr.len() - 1..0 {
    //     println!("{}", arr[a]); // print array elements in reverse order
    // }
    // for a in (0..arr.len()).rev() {
    //     println!("{}", arr[a]); // print array elements in reverse order
    // }

    // {
    //     let a = 10;
    //     println!("{}", a);
    // }
    // println!("{}", a); // error: variable a is not in scope

    // let str1 = String::from("Hello");
    // let str2 = str1;
    // println!("{}", str1); // error: value borrowed here after move
    // println!("{}", str2); // no error
    // let str3 = str2.clone();
    // println!("{}", str3); // no error

    // let str1 = String::from("Hello");
    // let str2 = str1;
    // println!("{}\n{}", str1, str2); // no error

    // let s1 = String::from("Ajay");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let s = "hello world";
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // let all = &s[..];
    // let world2 = &s[6..];
    // let hello2 = &s[..5];
    // println!("{}\n{}\n{}", hello, world, all);
    // println!("{}\n{}", world2, hello2);

    // #[derive(Debug)]
    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    // let mut user1 = User {
    //     username: String::from("Ajay"),
    //     email: String::from("ajaysathvik@outlook.com"),
    //     sign_in_count: 1,
    //     active: true,
    // };
    // let user2:User;
    // user2 = User {
    //     username: String::from("Sathvik"),
    //     email: String::from(""),
    //     sign_in_count: 0,
    //     active: false,
    // };
    // println!("{:?f}",user1); // print the structure user1 with debug trait {:?}
    // user1.username = "Ajay Sathvik".to_string();
    // println!("{} \n{} \n{} \n{}", user1.username, user1.email, user1.sign_in_count, user1.active);

    /* DAY8 */
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    //     fn can_hold(&self, other: &Rectangle) -> bool {
    //         self.width > other.width && self.height > other.height
    //     }
    //     fn square(size: u32) -> Rectangle {
    //         Rectangle {
    //             width: size,
    //             height: size,
    //         }
    //     }
    // } // impl is used to define methods for a structure
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };
    // println!("The area of the rectangle is {} square pixels.", rect1.area());
    // println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    // println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    // let sq = Rectangle::square(3);
    // println!("The area of the square is {} square pixels.", sq.area());

    // struct Rectangle{
    //     width:u32,
    //     length:u32,
    // }
    // let rect1 = Rectangle{
    //     width:30,length:30,
    // };
    // println!("The area of the rectangle is {} square pixels.",area(&rect1));
    // fn area(rectangle :&Rectangle)-> u32{
    //     rectangle.width * rectangle.length
    // }
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.length
    //     }
    //     fn perimeter(&self) -> u32{
    //         (2*self.width) + (self.length*2)
    //     }
    // }
    // println!("The area of the rectangle is {} square pixels.",rect1.area());
    // println!("The perimeter of the rectangle is {} square pixels.",rect1.perimeter());

    // #[derive(Debug)]
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // struct IpAddr{
    //     kind:IpAddrKind,
    //     address:String,
    // }
    // let home = IpAddr{
    //     kind:IpAddrKind::V4,
    //     address:String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr{
    //     kind:IpAddrKind::V6,
    //     address:String::from("::1"),
    // };
    // println!("{:?}\n{:?}",home.kind,home.address);

    // let mut v: Vec<i32> = Vec::new(); // create a new vector v of type Vec<i32>
    // v.push(5); // push 5 into the vector v
    // v.push(6); // push 6 into the vector v
    // v.push(7); // push 7 into the vector v
    // v.push(8); // push 8 into the vector v

    // let mut vec = vec![1, 2, 3]; // create a new vector v of type Vec<i32> with values 1,2,3
    // vec.push(69);
    // vec.pop();
    // let first: &i32 = &vec[0]; // get the first element of the vector
    // println!("The third element is {}", first);
    // let third: Option<&i32> = vec.get(2); // get the third element of the vector
    // match third {
    //     Some(i) => println!("The third element is {}", i),
    //     None => println!("There is no third element."),
    // }

    // let mut s1 = String::from("hello");
    // let mut s2 = String::from("world");
    // let mut s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // let mut score: HashMap<String, u32> = HashMap::new(); // create a new hashmap score
    // score.insert(String::from("Blue"), 10); // insert a new key-value pair into the hashmap
    // score.insert(String::from("Yellow"), 50); // insert a new key-value pair into the hashmap
    // println!("{:?}", score); // print the hashmap score

    /* Day 9 */
    // Generics in Rust
    //it is used to reduce the code duplication by using the same code for different data types
    // fn generic_add<t>(a:t,b:t)->t{
    //     a+b // t is a generic type which can be any data type
    // }

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // struct Pointint{
    //     x:i32,
    //     y:i32,
    // }
    // let integer = Pointint{x:5,y:10};
    // struct Pointfloat{
    //     x:f32,
    //     y:f32,
    // }
    // let float = Pointfloat{x:5.0,y:10.0};
    // struct Pointgeneric<T>{
    //     x:T,
    //     y:T,
    // }
    // let integer = Pointgeneric{x:5,y:10};
    // let float = Pointgeneric{x:5.0,y:10.0};

    // struct Point<T, U,V> {
    //     x: T,
    //     y: U,
    //     z: V,
    // }
    // let both_integer = Point { x: 5, y: 10, z: 15.0 };
    // let both_float = Point { x: 5.0, y: 10.0, z: 15 };
    // let integer_and_float = Point { x: 5, y: 10.0, z: 15 };

    // Traits in Rust
    // it is used to define the behaviour of a data type or a structure or an enum or a function or a method or a parameter or a return type or a variable or a constant
    // trait read {
    //     fn input()->i32;
    //     fn output()->i32;
    // }
    // impl read fn book(){
    //     fn input()->i32{
    //         // any code
    //     }
    //     fn output()->i32{
    //         // any code
    //     }
    // }

    // struct Circle {
    //     x: f64,
    //     y: f64,
    //     radius: f64,
    // }
    // impl HasArea for Circle {
    //     fn area(&self) -> f64 {
    //         std::f64::consts::PI * (self.radius * self.radius)
    //     }
    // }
    // let c1 = Circle {
    //     x: 0.0f64,
    //     y: 0.0f64,
    //     radius: 2.0f64,
    // };
    // print_area(c1);

    
    
}
// trait HasArea {
//     fn area(&self) -> f64;
// }
// fn print_area<T: HasArea>(Shape: T) {
//     println!("This shape has an area of {}", Shape.area());
// }

// fn largestnum(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largestchar(list: &[char]) -> char {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest<T:std::cmp::PartialOrd>(list:&[T]) -> &T{
//     let mut largest = &list[0];
//     for i in list{
//         if i > largest{
//             largest = i;
//         }
//     }
//     largest
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
