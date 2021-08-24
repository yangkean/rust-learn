fn main() {
    /* 什么是所有权 */

    // 所有权系统用于管理内存。
    // 所有权规则：
    // 1. 每个值都被它的所有者（owner）变量拥有
    // 2. 值在任意时刻都只能被一个所有者拥有
    // 3. 当所有者离开作用域，这个值将被丢弃

    // 作用域是一个项（item）在程序中有效的范围
    {
        // `s` 变量无效
        let s = "ell"; // `s` 变量开始有效

        // TODO sth with `s`
    } // `s` 变量变得无效

    // 两种字符串类型：字符串字面量，String 类型。
    // 字符串字面量是不可变的且必须在编写代码时就必须知道字符串的值，这个类型是储存在栈上的
    // String 类型储存在堆上，不需要提前知道字符串的大小且字符串可被修改
    let mut s = String::from("hello"); // 创建 String 类型的字符串
    s.push_str(", world!"); // 追加字符串
    println!("{}", s);

    // 移动（move）
    // 当持有堆中数据值的变量将值赋给另一个变量时移动它
    // let s1 = String::from("hh");
    // let s2 = s1; // Rust 使 s1 无效了，并且拷贝了 s1 上指向字符串内容（在堆上）内存的指针、一个长度、一个容量（这三个数据在栈上），即 s1 被移动到了 s2
    // println!("{}", s1); // 使用无效的引用会报错

    // Rust 永远不会自动创建数据的深拷贝（deep copy）

    // 克隆（clone）
    // 深度复制 `String` 中堆上的数据
    let s1 = String::from("hell");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝（copy）
    // 复制只在栈上的数据，既不是 clone，也不是移动，没有深浅拷贝的区别
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 函数返回值可以用来转移所有权

    /* 引用和借用 */

    // 引用（references）可以让我们在函数传参时使用一个值但不获取所有权
    // 我们用符号 `&` 来创建一个指向 s1 的引用，允许你使用 s1 但不获取 s1 的所有权
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    change(&mut s1);
    println!("The s1 is {}.", s1);

    // 在特定作用域中的特定数据有且只有一个可变引用
    // 这一限制的好处是 Rust 可以在编译时就避免数据竞争（data race）
    // 像下面这样会报错
    // let r1 = &mut s1;
    // let r2 = &mut s1;

    // 不能在拥有不可变引用的同时拥有可变引用
    // 像下面这样会报错
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;

    // 在存在指针的语言中，保留指向某块内存的指针的同时释放内存很容易错误地生成一个悬垂指针（dangling pointer），这种指针是指其指向的内存可能已经被分配给其他持有者
    // let reference_to_nothing = dangle();

    /* Slices */
    
    // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合
    // 使用 `[starting_index..ending_index]` 指定的 range 创建一个 slice，其中 starting_index 是包含 slice 的第一个位置（为 0 时可省略），ending_index 是最后一个位置的前一个值（slice 包含集合的最后一个元素时也可省略），同时省略首尾数字即获取整个集合的引用
    // 字符串 slice 的类型签名写作 `&str`
    let s = String::from("hello world");
    let hello = &s[0..5]; // 获取 String 中一部分值得引用
    let world = &s[6..11];
    let first_word_result = first_word(&s[..]);
    println!("slice: {} {}, first_world: {}", hello, world, first_word_result);
}

// 函数签名使用 & 来表明参数 s 是一个引用
// 引用离开作用域后并不丢弃它指向的数据，因为我们并没有指向的数据的所有权
// 我们称“把引用作为函数参数”为借用（borrowing）
// 引用指向的值默认是不可修改的
fn calculate_length(s: &String) -> usize {
    // s.push_str("kk"); // 报错
    s.len()
}

// 创建可变引用
fn change(s: &mut String) {
    s.push_str(" world");
}

// 野指针(wild pointer)就是没有被初始化过的指针
// 垂悬指针（dangling reference）: 指针指向的位置内存已被回收，但是对该指针没有作任何的修改（置 NULL），以至于该指针仍旧指向已经回收的内存地址。会引发不可预知的错误
// s 在 dangle 函数中创建，当该函数执行完毕后， s 将被释放
// 但我们尝试返回了它的引用，这个引用指向的是一个无效的 String，Rust 会在编译时报错
// fn dangle() -> &String {
//     let s = String::from("hhh");

//     &s
// }

// 获取 String 中第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // as_bytes 将 String 转化为字节数组

    // iter 在字节数组上创建迭代器，enumerate 包装 iter 的结果并返回一个元组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
