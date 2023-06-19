/// ## 知识点
///
/// ### 作用域规则 scope-rules of RAII
///
/// > RAII(resource acquisition is initilization) 资源获取即初始化
///
/// rust 不仅会在 栈内存上保存数据，也会在其他地方保存数据，譬如：Box<T> 会在堆内存上保存数据
///
/// Rust 中所有对象都有作用域，且强制执行 RAII，即当对象离开作用域时，会调用它的析构函数。它的内存就被释放掉
///
/// ### 析构函数
///
/// Rust 中的析构函数时通过 `Drop` trait 来实现的
///
/// 无需为每种类型都实现 `Drop` 仅需要对那些在资源释放时有自己逻辑的类型实现即可
///
///
fn create_box() {
    let _box3 = Box::new(32i32);

    // bb 在此处被销毁，占用资源被释放
}

fn main() {
    let _box1 = Box::new(52i32);

    {
        let _box2 = Box::new(56i32);
        // _box2 在此处被释放
    }

    for _ in 0i32..1000 {
        create_box()
    }

    let _x: TestDrop = TestDrop;
    println!("Made a testDrop!");

    // _box1 在此处被销毁
}

struct TestDrop;

impl Drop for TestDrop {
    fn drop(&mut self) {
        println!("execute drop method")
    }
}
