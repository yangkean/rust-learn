// `->` 声明函数返回值的类型
fn hello() -> u32 {
    return 1;
}

const MAX_NUMBER: u32 = 200_000;
// const HH: u32 = hello(); // 报错

fn main() {
    /* 变量和可变性 */

    let kk = hello();
    println!("The value of kk is: {}", kk);

    // 常量与不可变变量区别：
    // 1. 不允许对常量使用 mut：常量不光默认不能变，它总是不能变
    // 2. 声明常量使用 const 关键字而非 let，而且必须注明值的类型
    // 3. 常量可以在任何作用域中声明，包括全局作用域，但变量（包括不可变变量）只能在函数中使用
    // 4. 常量只能用于常量表达式，而不能作为函数调用的结果，或任何其他只在运行时计算的值
    const MAX_POINTS: u32 = 100_000; // Rust 中可以使用 `_` 分隔符划分一个长数字而不影响数字的值，这样便于读数

    // 使用大型数据结构时，适当地使变量可变，可能比复制和返回新分配的实例更快。
    // 对于较小的数据结构，总是创建新实例，采用更偏向函数式的风格编程，可能会使代码更易理解，为可读性而遭受性能损失或许值得
    let mut x = 5;
    println!("The value of x is: {}, outside constant is: {}", x, MAX_NUMBER);
    x = 6;
    println!("The value of x is: {}, inside constant is: {}", x, MAX_POINTS);

    // 覆盖（shadowing），当多次用 let 定义一个和之前变量名重名的新变量时，新变量会覆盖之前的变量
    // 与 mut 的区别：
    // 1. 需要使用 let 关键字，直接赋值会在编译时报错
    // 2. 再次使用 let 时，实际上创建了一个新变量，可以改变值的类型，从而复用了这个名字。而 mut 定义的变量是不可以改变类型的
    let y = 666;
    let y = y + 1;
    let y = y + 3;
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of y is: {}", y);
    println!("The value of spaces is: {}", spaces);

    /* 数据类型 */

    // Rust 是静态类型语言，也就是说在编译时就必须知道所有变量的类型
    // 內建数据类型：标量（scalar）和复合（compound）
    // 标量：代表一个单独的值。Rust 的四种基本标量类型：整型、浮点型、布尔型、字符型。
    // 关于整型，`有符号数`以二进制补码形式存储。每一个有符号的变体可以储存包含从 -(2^(n-1)) 到 2^(n-1) - 1 在内的数字，这里 n 是有符号二进制数的位数，无符号数可以储存从 0 到 2^n - 1 的数字
    // Rust 中数字类型默认是 i32
    // 关于浮点型，有 `f32` 和 `f64`，默认类型是 `f64`
    // 关于字符型，字符类型代表了一个 Unicode 标量值
    // 复合：可以将多个其他类型的值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）
    let a: u32 = 8888;
    let b: f32 = 3.0;
    let c: bool = true;
    let d: char = '🐈';
    // 使用一个括号中的逗号分隔的值列表来创建一个元组，这些值得类型可以是不相同的
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构赋值
    println!("The value of z is: {}", z);
    println!("The value of y is: {}", tup.1); // 可以使用点号 `.` 后跟值得索引来直接访问元组的值
    // Rust 中数组的每个元素的类型必须相同，数组是固定长度的，一旦声明，他们的长度不能增加或缩小
    let arr = [1, 2, 4, 5];
    println!("The value of arr[2] is: {}", arr[2]);

    /* 函数如何工作 */

    // 函数体由一系列的语句和一个可选的表达式构成
    // 语句（statements）是执行一些操作但不返回值的指令，表达式（expressions）计算并产生一个值。
    // 创建作用域的大括号（代码块）`{}` 是一个表达式
    another_function(5);
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 与语句不同，表达式并不包含结尾的分号，若加上分号，则使这一行变为语句，也就不返回值了
    };
    println!("The value of y is: {}", y);

    /* 控制流 */

    let number = 3;
    // Rust 只会执行第一个条件为真的代码块，并且它一旦找到一个以后，就不会检查剩下的条件了（在有多重条件时）
    if number < 5 { // if 表达式的条件必须是 `bool` 类型
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 因为 if 是一个表达式，所以可以在 `let` 语句右侧使用它
    // `if` 的每个分支的可能的返回值都必须是相同类型
    let number = if true {
        8
    } else {
        99
    };
    println!("The value of number is: {}", number);

    // loop 循环
    let mut count = 5;
    loop {
        count -= 1;
        println!("again");
        if count <= 0 {
            break;
        }
    };

    // while 循环
    count = 5;
    while count > 0 {
        count -= 1;
        println!("again1");
    };

    // for 循环
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// another_function 既可以在 main 函数之后定义，也可以在 main 函数之前，只要定义了就可以调用，Rust 不关心函数定义于何处
// 必须声明每个参数的类型
// 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值
fn another_function(x: i32) {
    println!("Another {}", x);
}
