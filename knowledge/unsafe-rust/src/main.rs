use std::slice;

// 全局变量，也叫静态变量
// 不可变静态变量和常量的区别在于，静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。常量则允许在任何被用到的时候复制其数据
// 常量与静态变量的另一个区别在于静态变量可以是可变的
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 修改一个可变静态变量是不安全的
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);

    // 引用外部代码是不安全的，因为它们并不遵守rust的规定
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 提供函数给C调用
    #[no_mangle] // `#[no_mangle]`告诉rust别改变函数名用于其他的编译处理
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    add_to_count(4);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 引用外部代码（其他编程语言写的）
extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
