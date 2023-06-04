/// ## 知识点 发散函数（diverging functions）
///  
/// > 采用 `!` 符号进行返回值，该值不能被实例化，所以该值具有所有可能的空值的集合，他与 `()` 一定会有一个值返回给用户不同
///

fn return_empty_fn() {
    ()
}

fn main() {
    judge_dot();
}

fn judge_dot() {
    // return () 照常返回
    let _a: () = return_empty_fn();
    println!("run with 17 line");
    let _b = panic!("just panic");
    println!("run with 21 line");
}
