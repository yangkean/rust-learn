fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (3, 5);
    print_coordinates(&point);

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // match 表达式会新建一个作用域，里面的变量会shadow外部的同名变量
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 2;
    match x {
        // `|`语法表示or，即或的意思
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        // x是1，2，3，4，5都可，`..=`表示包含，只用于操作数值或字符
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // let Point { x, y } = p;
    // assert_eq!(0, x);
    // assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // 如果一个变量未被使用，rust会发出警告，这个时候在变量前使用 `_`可以忽略这个警告
    let _x = 5;
    let y = 10;

    let origin = ThreePoint { x: 0, y: 0, z: 0 };
    match origin {
        // 使用`..`忽略剩余值
        // ThreePoint { x, y: _, z: _ } => println!("x is {}", x),
        ThreePoint { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(5);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("here {}", x),
        None => (),
    }

    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            // @ 创建一个变量并捕获符合模式的值
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum HelloMessage {
    Hello { id: i32 },
}

struct ThreePoint {
    x: i32,
    y: i32,
    z: i32,
}

// `_`可以用于忽略一个值，在任何模式中都可以使用，并且`_`不会绑定任何值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
