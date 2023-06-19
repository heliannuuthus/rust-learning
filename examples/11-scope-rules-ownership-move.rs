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
///
fn move_box_point(owner: Box<u32>) {
    println!("obtain ownership: {}", owner)
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
}
