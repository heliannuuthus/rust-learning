/// ## 知识点
///
/// ### 所有权和移动 ownership and move
///
/// Rust 中变量负责释放他们拥有的资源，每个资源允许有且仅有一个拥有者。这也防止了资源重复释放
///
/// 但是并非所有的变量都有资源，类似像 `引用` 这样的变量就没有资源
///
/// 通过赋值或者通过值的参数传递方式可以将资源移动到另一个 owner 手里
///
/// 在资源发生转移后，原来的拥有者不可再使用这份资源，防止了悬挂指针的产生
///
/// > 可变性：当所有权转移时，数据的可变性可能发生改变
///
/// > 部分移动：当一个结构体内部变量发生了 `by_reference` 或 `by_move` 的移动（解构），该结构体整体不可在后续被使用
/// 未发生移动的引用部分仍可使用，
///
///
fn move_box_point(owner: Box<u32>) {
    println!("obtain ownership: {}", owner)
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
fn main() {
    // 栈分配内存 3 整型
    let x = 3u32;

    // 将 x 复制一份给 y，由于是栈内存，所以不存在资源移动
    let y = x;

    // 互相使用，互不影响
    println!("x: {}, y: {}", x, y);

    let _box = Box::new(52u32);

    // 将 _box 指针地址分配给 box2 现在 box2 和 _box 都指向同一资源，但是仅有 box2 拥有它
    let box2 = _box;

    // _box.as_mut();
    // ownership move to move_box_pointer
    move_box_point(box2);

    // error
    // box2.as_mut();

    // 可变性
    // 创建智能指针
    let unmutable_x = Box::new(32u32);

    // 不可变修改
    // *unmutable_x = 4;

    let mut mutable_x = unmutable_x;

    // 可变修改
    *mutable_x = 42;

    println!("mutable_x: {}", mutable_x);

    // 部分移动

    let person = Person {
        name: String::from("name"),
        age: 32
    };
    // 发生解构，age 仅是 引用，而 name 剥夺了 person 内 name 的所有权
    let Person{name , ref age} = person;
    println!("解构生成的变量 name: {}", name);
    println!("解构生成的变量 age: {}", age);
    // println!("person 整体不能被使用: {:?}", person);
    println!("仅可使用 age: {}", person.age)
}
