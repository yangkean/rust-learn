// collections 类型的结构数据都是存在堆里的。vector、string、hash map都是常见的collection
// vector允许存储一系列相互邻接的值
// string是字符的集合
// hash map是键和值关联的结构
fn main() {

    /**************
     * Vector
     **************/

    // 声明空的 vector
    let v: Vec<i32> = Vec::new();
    // 使用 `vec!` 宏命令快速创建 vector
    let v2 = vec![1, 2, 3];

    // 更新vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // 使用下标或者get获取vector的值
    let val: &i32 = &v2[2];
    println!("The third element is {}", val);
    match v2.get(2) {
        Some(value) => println!("The third element is: {}", value),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v2[100]; // 越界访问导致程序退出
    let does_not_exist = v2.get(100); // 越界访问不会导致程序退出

    println!("run here");

    // 遍历一个vec 
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vector只能存储相同类型的值，但借助 enum 的力量可以去声明不同的类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /**************
     * String
     **************/
    
    // 空string
    let mut s = String::new();
    // string 常量转 String 类型
    let s = "initial contents".to_string();
    // 创建string
    let s = String::from("initial contents");
    println!("s is {}", s);

    // String 类型是可以改变内容的，与string常量不同

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    // 添加单个字符
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 两个 `&str` 不能相加，两个 `String` 也不能相加，只能是 `String + &str/&String(会被强制转换为 &str)`
    // 原因：https://github.com/rust-lang/rust/issues/39018，https://zhihu.com/question/478127148/answer/2053155100
    // + 操作符使用的是 add 方法
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let s4 = "a".to_string() + "b";
    println!("s4 is {}", s4);

    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");
    println!("format is {}", format!("{}-{}-{}", x, y, z));

    // "Здравствуйте" 字符串每个字符是用两个字节存储的
    let hello = "Здравствуйте";
    // s 是 &str 类型，包含hello字符串开始的四个字节
    let s = &hello[0..4];
    println!("slice is {}", s);

    // 是用 chars 方法获得字符串的所有字符
    println!("all chars: {:#?}", "नमस्ते".chars());
    for c in "नमस्ते".chars() {
        println!("印地字符：{}", c);
    }

    // bytes 方法返回所有字节值
    println!("all bytes: {:?}", "नमस्ते".bytes());
    for b in "नमस्ते".bytes() {
        // println!("{}", b);
    }

    /**************
     * Hash Map
     **************/
    
    use std::collections::HashMap;

    // hash map 存储在堆中
    // key必须是同一种类型，value必须是同一种类型
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores is {:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 用下划线表示HashMap的key和value的类型占位，具体类型会由Rust推导出
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 覆盖值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // // 只在key没有值的时候插入值
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);
}
