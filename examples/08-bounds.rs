use std::fmt::Display;

/// ## 知识点
/// ### 约束（bounds）
///
/// > 当使用范型时通常会将类型参数指定为某一个 trait，当指定为这个 trait 之后我们可以称之为对该范型有一个 **约束**
///
///
///

// 此时调用 printer 方法的必须是实现了 Display trait 的对象
fn printer<T: Display>(t: T) {
    println!("{}", t)
}

struct S<T: Display>(T);

fn main() {
// Vec 没有实现 Display trait 所以传入 S(vec!()) 会报错
let s = S(vec!["1", "1"]);

}
