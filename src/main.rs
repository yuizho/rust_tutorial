fn main() {
    print_sum(5, 6);
    println!("{}", add_one(1));
    // let f: fn(i32) -> i32 = add_one;
    let f = add_one;
    println!("{}", f(1));

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    let x = 5;
    if x == 5 {
        println!("xは5です")
    }
    let y = if x == 5 { 10 } else { 15 };
    println!("{}", y);

    let mut i = 5;
    let mut done = false;
    while !done {
        i += i - 3;
        println!("{}", i);

        if i % 5 == 0 {
            done = true;
        }
    }

    for x in 0..10 {
        println!("{}", x);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    //    let (v1, v2, answer) = borrowing1(v1, v2);
    //    println!("{:?}", v1);

    let answer = borrowing2(&v1, &v2);
    println!("{}", answer);

    let mut x = 5;
    {
        let y = &mut x; // -+ &mut借用がここから始まる
        *y += 1; //  |
    } // -+ ... そしてここで終わる
    println!("{}", x); // <- ここでxを借用しようとする

    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // v.push(34); cannot borrow `v` as mutable
    }

    //let y = &5;
    let f = Foo { x: &5 };
    println!("{:?}", f.x);

    let x: &'static str = "Hello, world.";
    println!("{:?}", x);

    use std::sync::Arc;
    let x = Arc::new(5);
    let y = x.clone(); // 5の& T を配る
    println!("{:?}", y);

    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("something else"),
    }

    let msg: Message = Message::Write("xxx".to_string());
    match msg {
        Message::Quit => println!("quit"),
        _ => println!("something else"),
    }
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {
        self.x
    }
}

fn borrwoing1(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}

fn borrowing2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    42
}

fn print_sum(x: i32, y: i32) {
    println!("sum is : {}", x + y)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("this function never return!");
}
