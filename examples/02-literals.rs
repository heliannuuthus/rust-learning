/// ## 知识点
/// 对于数字字面量表示使用 {value}{type} 将类型作为后缀加上去，比如 520 的字面量可以为 520u16
/// rust 对添加后缀的字面量有一套默认的规则，整数 -> i32 浮点数 -> f64
use std::mem;

fn main() {
    // 带后缀的字面量类型在初始化时就知道了
    let x = 1u8;
    let y = 2i8;
    let z = 3.0f32;

    // 无后缀使用编译器默认指定的值
    let w = 32;
    let e = 2.6;

    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `w` in bytes: {}", mem::size_of_val(&w));
    println!("size of `e` in bytes: {}", mem::size_of_val(&e));
}
