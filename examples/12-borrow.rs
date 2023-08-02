/// ## 知识点
///
/// ### 借用 (borrow)
///
/// Rust 中，大多数时候我们都倾向于访问数据的同时，不获取他们的所有权。
///
/// 为了达到这个目的，Rust 使用了 借用(borrowing) 规则实现。
///
/// 对象可以通过 &T 借用，来传递，从而取代 T 直接使用值传递
///
/// 编译器通过借用检查静态地保证了 引用 总是指向有效的对象，也就是说，该引用指向一个对象时，该对象不能被销毁
///
/// ### 别名使用
///
/// > 同一时间仅能存在 **一次** 可变借用，仅当最后一次可变借用被使用后，才能产生不可变借用
///
/// ### ref 模式
///
/// > 通过 let 绑定来进行模式匹配或者解构变量的时候，`ref` 关键字可以用来创建结构体/元组字段的引用
///
///
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
fn move_ownership(box1: Box<u32>) {
    println!("move a box, and destroy it: {:?}", box1)
}

fn borrow_reference(box1: &u32) {
    println!("borrow a box, and use it: {:?}", box1)
}
fn main() {
    let box1 = Box::new(32u32);
    let box2 = 42;
    borrow_reference(&box1);
    let ref s = box2;
    borrow_reference(s);
    borrow_reference(&box2);

    {
        let _ref_to_32 = Box::new(123u32);

        // error
        // move_ownership(_ref_to_32);

        // 此处 borrow _ref_to_32，由于 move_ownership 导致 _ref_to_32 发生了转移，所以后续不能再 borrow _ref_to_32
        borrow_reference(&_ref_to_32);
    }
    move_ownership(box1);

    // ref 模式

    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 在解构一个结构体时 `ref` 同样有效。
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x
    };
}
