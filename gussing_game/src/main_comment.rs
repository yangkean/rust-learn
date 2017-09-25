// 注释版

extern crate rand; // 通知 Rust 我们要使用外部依赖，这也会调用相应的 use rand，所以现在可以使用 rand:: 前缀来调用 rand crate 中的任何内容

// Rust 默认只在每个程序的 prelude 中引入少量类型。
// 如果需要的类型不在 prelude 中，你必须使用一个 use 语句显式的将其引入作用域
use std::io; // 引入标准库（std）中的 io 库到当前作用域中用于获取用户输入
use std::cmp::Ordering; // 引入用于比较的类
use rand::Rng; // Rng 是一个 trait，它定义了随机数生成器应实现的方法 ，想使用这些方法的话此 trait 必须在作用域中

fn main() {
    println!("Guess the number!");

    // rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程，并从操作系统获取 seed
    // 接下来，调用随机数生成器的 gen_range 方法。这个方法由刚才引入到作用域的 Rng trait 定义。
    // gen_range 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数。它包含下限但不包含上限
    let secret_number = rand::thread_rng().gen_range(1, 101); // 得到随机数

    // 循环
    loop {
        println!("Please input your guess.");

        // let 语句用于创建变量
        // 借助 Rust 的类型推断，此处不需要我们声明变量类型
        // 在 Rust 中，变量默认是不可变的，我们可以在变量名前使用 mut 来使一个变量可变
        // `String::new()` 会返回一个 String 的新实例，String 是一个标准库提供的字符串类型。
        // ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
        // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为 静态方法（static method）。
        let mut guess = String::new(); // new 函数创建了一个新的空 String

        // stdin 函数返回一个 std::io::Stdin 的实例，std::io::Stdin 是代表终端标准输入句柄的类型
        // .read_line(&mut guess) 调用 read_line 方法从标准输入句柄获取用户输入
        // 参数 &mut guess 中的 & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        // 用户在标准输入键入的内容会存入 guess 参数中
        // 另外，参数默认是不可变的，需要写成 &mut guess 而不是 &guess 来使其可变
        // io::Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。
        // 如果 io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // read_line 返回一个 io::Result 实例
        
        // 创建了一个叫做 guess 的变量
        // Rust 允许我们用一个新值覆盖（shadow）guess 之前的值
        // 这个功能常用在需要转换值类型之类的场景，它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量
        // String 实例的 trim 方法会去除字符串开头和结尾的空白，包括换行符 \n
        // 字符串的 parse 方法 将字符串解析成数字。因为这个方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型，这里通过 let guess:u32 指定，即 32 位无符号整数，Rust 默认使用 i32，即 32 位整数
        // guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型
        // parse 方法返回一个 Result 类型，Result 是一个拥有 Ok 或 Err 成员的枚举。像之前 “使用 Result 类型来处理潜在的错误” 讨论的 read_line 方法那样调用 expect 即可，比如无法转换为数字时便会出错
        let guess:u32 = match guess.trim().parse() {
            // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。
            // 这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num
            Ok(num) => num,
            // 如果 parse 不 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err
            // _ 是一个通配值，本例中用来匹配所有 Err 值，不管其中有何种信息
            Err(_) => continue,
        };
            // .expect("Please type a number!");
        
        // 第一个参数是格式化字符串，里面的 {} 是预留在特定位置的占位符
        println!("You guessed: {}", guess);

        // cmp 方法用来比较两个值并可以在任何可比较的值上调用
        // 它获取一个被比较值的引用
        // 使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么
        // 一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）以及表达式开头的值与分支模式相匹配时应该执行的代码
        // 正确匹配时表达式就会终止，否则检查每个分支
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break; // 退出循环
            }
        }
    }
}
