use std::thread;
use std::time::Duration;

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
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
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 3, 5];
    // closure 中可以访问它定义位置所处的环境或者说作用域中的变量
    let equal_to_x = |z| z == x;
    // let equal_to_x = move |z| z == x; // 使用move关键字后可以将变量所有权移走
    let y = vec![1, 3, 5];
    assert!(equal_to_x(y));
    println!("x is {:?}", x);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
