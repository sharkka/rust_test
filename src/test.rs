/**
 * brief  : test code
 *          RUST
 * date   : 2022-09-26 15:19:11
 * author : ANYZ
 */
pub mod testbox {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    fn test_serde() {
        let point = Point { x: 1, y: 2 };
        let serialized = serde_json::to_string(&point).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
    use std::{error::Error, io::Read};

    #[allow(unused)]
    async fn test_reqwest() -> Result<(), Box<dyn Error>> {
        use reqwest;
        use std::time::Duration;
        let client = reqwest::Client::new();
        let doge = client
            .get("rust.org")
            .header("Accept", "text/plain")
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .text()
            .await?;
        println!("doge = {:}", doge);
        Ok(())
    }

    fn sum_of_squares(input: &[i32]) -> i32 {
        input.iter().map(|&i| i * i).sum()
    }

    fn test_rayon() {
        let mut v = vec![];
        for i in 1..10 {
            v.push(i);
        }
        println!("{}", sum_of_squares(&v));
    }

    fn test_std() {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    #[allow(unused)]
    async fn sleepus() {
        use async_std::task::sleep;
        for i in 1..=10 {
            println!("Sleepus {}", i);
            sleep(std::time::Duration::from_millis(500)).await;
        }
    }
    #[allow(unused)]
    async fn interruptus() {
        use async_std::task::sleep;
        for i in 1..=5 {
            println!("Interruptus {}", i);
            sleep(std::time::Duration::from_millis(1000)).await;
        }
    }
    #[allow(unused)]
    async fn test_async() {
        use async_std::task::spawn;
        let sleepus = spawn(sleepus());
        interruptus().await;

        sleepus.await;
    }

    #[allow(unused)]
    fn test_guess() {
        use rand;
        use rand::Rng;
        use std::cmp::Ordering;

        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..=101);
        println!("The secret number is: {}", secret_number);

        loop {
            println!("Please input your guess.");
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }

    fn test_any() -> i32 {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup;
        println!("tup x: {}, y: {}, z: {}", x, y, z);
        println!("tup y: {}", tup.2);

        let mut counter = 0;
        let r = loop {
            counter += 1;
            println!("counter: {}", counter);
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("result of loop {}, {}", counter, r);

        let array = [1, 3, 5, 7, 9];
        for i in (1..4).rev() {
            println!("array {}", i);
        }
        for i in array.iter().rev() {
            println!("iter rev: {}", i);
        }
        let s1 = String::from("hello");
        let s3 = s1.clone();
        let s2 = s1;
        println!("s2: {}, s3: {}", s2, s3);

        return 0;
    }

    fn test_change(s: &mut String) {
        s.push_str(", world");
    }

    fn test_change_string() {
        let mut s = String::from("Hello");
        test_change(&mut s);
        println!("change string : {}", s);
    }

    fn test_first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    #[derive(Debug)]
    struct MColor(i32, i32, i32);
    #[derive(Debug)]
    struct MPoint(i32, i32, i32);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn test_meta_struct() {
        let mcolor = MColor(1, 2, 3);
        let mpoint = MPoint(5, 6, 7);

        println!("{:?}, {:?}", mcolor, mpoint);
    }
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn square(width: u32) -> Rectangle {
            Rectangle {
                width,
                height: width,
            }
        }
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    fn test_struct() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
        println!("The rect : {:#?}", rect1);
        println!("area by method : {}", rect1.area());
        let square = Rectangle::square(300);
        println!("The square : {:#?}", square);
    }
    #[allow(unused)]
    enum IpAddrKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    #[allow(unused)]
    enum Message {
        Quit,
        Move {
            x: u32,
            y: u32,
        },
        Write(String),
        #[allow(unused)]
        ChangeColor(u32, u32, u32),
    }

    impl Message {
        #[allow(dead_code)]
        fn call(&self) {}
        fn print(&self) {
            println!("{:#?}", self);
        }
    }

    fn test_enum() {
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let lo = IpAddr::V6(String::from("0.0.0.0"));

        println!("home: {:#?}, lo: {:#?}", home, lo);
        let msg = Message::Write(String::from("write"));
        msg.print();
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn test_option() {
        #[allow(unused_variables)]
        let some_number = Some(5);
        #[allow(unused)]
        let some_string = Some("a string");
        #[allow(unused)]
        let absent_number: Option<i32> = None;
        println!("test option");

        let five = Some(5);
        #[allow(unused)]
        let six = plus_one(five);
        #[allow(unused)]
        let none = plus_one(None);
    }

    fn test_match() {
        #[derive(Debug)]
        #[allow(unused)]
        enum UsState {
            Alabama,
            Alaska,
        }
        #[derive(Debug)]
        #[allow(unused)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        let coin: Coin = Coin::Dime;
        match coin {
            Coin::Penny => println!("coin cost {}", 1),
            Coin::Nickel => println!("coin cost {}", 5),
            Coin::Dime => println!("coin cost {}", 10),

            Coin::Quarter(state) => {
                println!("coin cost {:?}!", state)
            }
        }
        let coin2 = Coin::Quarter(UsState::Alabama);
        #[allow(unused)]
        let mut count = 0;
        if let Coin::Quarter(state) = coin2 {
            println!("state quarter from {:?}!", state);
        } else {
            count += 1;
            println!("{}", count);
        }

        let some_u8 = 0u8;
        match some_u8 {
            1 => println!("u8 {}", 1),
            _ => (),
        }
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
    #[allow(unused)]
    fn test_crates() {
        use std::collections::*;
        use std::io::{self, Write};

        crate::common::vogel::print();
    }
    #[allow(unused)]
    pub async fn test_console() -> Result<(), Box<dyn Error>> {
        use console::Term;
        use std::thread;
        use std::time::Duration;

        let term = Term::stdout();
        term.write_line("hello console")?;
        thread::sleep(Duration::from_millis(2000));
        term.clear_line()?;
        Ok(())
    }

    fn test_owo_colors() {
        use owo_colors::OwoColorize;
        println!("my number is {:#x}!", 10.green());
        println!("my number is not {}!", 4.on_red());
    }

    fn test_collection() {
        use std::collections::HashMap;
        let mut vec1: Vec<i32> = Vec::new();
        vec1.push(4);
        vec1.push(5);
        vec1.push(6);
        let vec2 = vec![1, 2, 3];

        let v1: &i32 = &vec2[0];
        println!("first element {}", v1);
        match vec2.get(0) {
            Some(v1) => println!("match success first element is {}", v1),
            None => println!("there is no first element"),
        }
        let terms = vec![String::from("blue"), String::from("yellow")];
        let initial_scores = [10, 50];
        let scores: HashMap<_, _> = terms.iter().zip(initial_scores.iter()).collect();
        for (key, value) in &scores {
            println!("key {}, value {}", key, value);
        }
    }

    fn test_file() {
        use std::fs::File;
        use std::io::ErrorKind;

        let filepath = String::from("hello.txt");
        let f = File::open(&filepath);

        let mut f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&filepath) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                },
                #[allow(unused)]
                other_error => panic!("There was a problem opening the file: {:?}", error),
            },
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        println!("size: {}, {:?}", buf.len(), buf);
        {
            #[allow(unused)]
            let fp = File::open("hello.txt").unwrap();
            #[allow(unused)]
            let fp = File::open("hello.txt").expect("Failed to open hello.txt");
        }
    }

    use std::io;
    #[allow(unused)]
    fn read_username_from_file() -> Result<String, io::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut s = String::new();
        let filepath = String::from("hello.txt");
        #[allow(unused)]
        let f = File::open(&filepath)?.read_to_string(&mut s)?;
        Ok(s)
    }

    fn test_maperr() {
        use std::fs::File;
        use std::io::ErrorKind;

        let path = String::from("hello.txt");
        #[allow(unused)]
        let f = File::open(&path).map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(&path).unwrap_or_else(|error| {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opning the file: {:?}", error);
            }
        });
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: String,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    #[allow(unused)]
    pub fn notify(item: impl Summary) {
        println!("Breaking new! {}", item.summarize());
    }
    #[allow(unused)]
    pub fn notify2<T: Summary>(item: T) {
        println!("Breaking new! {}", item.summarize());
    }

    pub trait Display {
        fn display(&self) -> i32;
    }

    pub trait Debug {
        fn debug(&self) -> i32;
    }
    #[allow(unused)]
    fn some_func<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        let i: i32 = 5;
        i
    }
    #[allow(unused)]
    fn largest<T>(list: &[T]) -> T
    where
        T: PartialOrd + Copy,
    {
        let mut lg = list[0];
        for &item in list.iter() {
            if item > lg {
                lg = item;
            }
        }
        lg
    }
    #[allow(unused)]
    fn largest_ref<T>(list: &[T]) -> &T
    where
        T: PartialOrd,
    {
        let mut index = 0;
        let mut i = 0;
        for item in &list[..] {
            if *item > list[index] {
                index = i;
            }
            i += 1;
        }
        &list[index]
    }

    fn test_largest_ref() {
        let vec = vec![5, 2, 1, 9, 22, 78];
        let lg = largest_ref(&vec);
        println!("largest value ref : {}", *lg);
    }
    #[allow(unused)]
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, radom_number: u32) {
        use std::thread;
        use std::time::Duration;
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_millis(200));
            num
        });
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if radom_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    fn test_closures() {
        generate_workout(3, 7);
        let v1 = vec![1, 2, 3];
        #[allow(unused)]
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let total: i32 = v1.iter().sum();
        println!("total sum : {}", total);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        let my_shoes: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
        my_shoes
    }

    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 7,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_shoes = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_shoes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("iterator sum : {}", sum);
        let vs = Counter::new().zip(Counter::new().skip(1));
        for (a, b) in vs {
            println!("({}, {})", a, b);
        }
    }
    #[allow(unused)]
    fn test_audio_decode(buf: &mut [i32]) {
        let buffer: &mut [i32] = buf;
        let coefficients: [i64; 12] = [1; 12];
        let qlp_shift: i16 = 8;

        for i in 12..buffer.len() {
            let prediction = coefficients
                .iter()
                .zip(&buffer[i - 12..i])
                .map(|(&c, &s)| c * s as i64)
                .sum::<i64>()
                >> qlp_shift;
            let delta = buffer[i];
            buffer[i] = prediction as i32 + delta;
        }
    }

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
enum RcList {
    Cons(Rc<RefCell<i32>>, Rc<RcList>),
    Nil,
}

fn test_box_cons() {
    use List::{Cons, Nil};
    #[allow(unused)]
    let list = Cons(1,
      Box::new(Cons(2,
        Box::new(Cons(3,
          Box::new(Nil))))));
}
        
fn test_box_rc() {
    use RcList::{Cons, Nil};
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    #[allow(unused)]
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    #[allow(unused)]
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}

struct MyBox<T>(T);
use std::ops::Deref;

impl<T> MyBox<T> {
    #[allow(unused)]
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> 
  where T : Messenger {
    #[allow(unused)]
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    #[allow(unused)]
    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max > 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use  std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[allow(unused)]
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    #[allow(unused)]
    value: i32,
    parent: RefCell<Weak<Node>>,
    #[allow(unused)]
    children: RefCell<Vec<Rc<Node>>>,
}

fn test_weak_rc() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn test_weak_rc2() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong ={}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
      let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
      });
      *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
      println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
      );
      println!(
        "leaf strong = {}, weak  {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
      );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn test_spawn() {
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn test_channel_msg() {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
#[allow(unused)]
fn test_channel_multi_prouctor() {
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vs = vec![
            "hi",
            "from",
            "send",
            "more",
        ];
        for v in vs {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vs = vec![
            "good",
            "day",
            "commando",
            "yes",
        ];
        for v in vs {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for res in rx {
        println!("Got {}", res);
    }
    
}

fn test_mutex() {
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
      let mut num = m.lock().unwrap();
      *num = 6;
    }
    println!("m = {:?}", m);
}

fn test_thread_arc() {
    use std::thread;
    use std::sync::{Mutex, Arc};

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {

    }
}

pub struct Screen {
    pub conponents: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for item in self.conponents.iter() {
            item.draw();
        }
    }
}
struct SelectBox {
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32,
    #[allow(unused)]
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {

    }
}

fn test_derive_feature() {
    let screen = Screen {
        conponents: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 20,
                height: 30,
                label: String::from("ok"),
            }),
        ]
    };
    screen.run();
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) ->&'a str {
        ""
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post{
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published  {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) ->&'a str {
        &post.content
    }
}

fn test_while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

enum CColor {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum CMessage  {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(CColor),
}

fn test_alot_match() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let y = 9;
    match y {
        1 ..=5 => println!("one through five"),
        _       => println!("something else"),
    }

    let p = Point {x: 0, y: 7};
    let Point {x:a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let msg = CMessage::ChangeColor(CColor::Hsv(0, 160, 255));

    match msg {
        CMessage::ChangeColor(CColor::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,g,b
            )
        },
        CMessage::ChangeColor(CColor::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,s,v
            )
        }
        _ => ()
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
      .iter()
      .map(|&Point {x, y} | { x * x + y * y })
      .sum();

    println!("points sum: {}", sum_of_squares);
    #[allow(unused)]
    let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: 10});


}
#[allow(unused)]
fn test_allmatch(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn test_part_parameter() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value == new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let _abc = 5;

    let origin = Point3D {x: 0, y: 0, z: 0};
    match origin {
        Point3D { x, .. } =>  println!("x is {}", x),
    }
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn test_match_at_var() {
    enum Message {
        Hello { id: i32},
    }

    let msg = Message::Hello {id: 5};
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello { id: 10 ..= 12 } => {
            println!("Found an id anthor range");
        },
        Message::Hello {id} => {
            println!("Found some other id: {}", id);
        }
    }
}

fn test_match_ref() {
    let robot_name = &Some(String::from("Bors"));
    match robot_name {
        &Some(ref name) => println!("Found a name: {}", name),
        None => (),
    } 
}

fn test_unsafe_code_01() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

use std::slice;

fn test_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) ->i32;
}

fn test_extern_func() {
    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }
}

struct Context<'a>(&'a str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

struct Ref<'a, T: 'a>(&'a T);

fn test_advanced_func() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}













    pub fn test_all() {
        test_serde();
        //let _result = test_reqwest().await;
        test_rayon();
        test_std();
        //test_async().await;
        //test_guess();
        test_any();
        test_change_string();
        let s = String::from("you folish");
        let sb = test_first_word(&s);
        println!("sb {}", sb);
        let email = String::from("abc@gmail.com");
        let username = String::from("tom");
        let user = build_user(email, username);
        println!(
            "user email {}, sign_in_count {}, active: {}",
            user.email, user.sign_in_count, user.active
        );

        let user1 = User {
            email: String::from("b@c.com"),
            username: String::from("user1"),
            active: true,
            sign_in_count: 2,
        };
        let user2 = User {
            username: String::from("user2"),
            ..user1
        };
        println!("user2 {}", user2.username);
        test_meta_struct();
        test_struct();
        test_enum();
        test_option();
        test_match();
        test_crates();
        test_owo_colors();
        test_collection();
        test_file();
        test_maperr();
        test_largest_ref();
        test_closures();
        filters_by_size();
        using_other_iterator_trait_methods();
        test_box_cons();
        test_box_rc();
        test_weak_rc();
        test_weak_rc2();
        test_spawn();
        test_channel_msg();
        test_mutex();
        //test_channel_multi_prouctor();
        test_thread_arc();
        test_derive_feature();
        test_while_let();
        test_alot_match();
        test_part_parameter();
        test_match_at_var();
        test_match_ref();
        test_unsafe_code_01();
        test_extern_func();
    }
}
