/// ## 知识点
/// > rust 中的闭包（closeure）也叫 lambda 表达式，一种可以捕获周围作用域内环境变量的函数
///   在临时使用时非常的方便，不用自己定义一个 fn 然后传入
///
/// > 调用闭包时，它的入参和返回值都 **可以** 自动推导，但是 **变量名** 必须指定
/// 特点：
/// - `||` 替代 `()` 将入参包裹起来
/// - 当仅存在单个表达式，某一句话（用一个 ；可以表示）时可以省略  `{}`
/// - 有能力捕获周围环境变量
///
/// 切记，将闭包绑定在某一个引用上是不确定的说法，闭包产生的类型就是闭包类型，不是传统的引用类型，并且无法对闭包函数进行解引用
///
/// ### capturing （捕获）
/// 闭包优先使用引用来捕获变量，仅在需要时使用其他的方式
///
/// > 闭包在 borrow 一个变量且在闭包函数内对其进行修改后会导致闭包函数内部发生改变，故而需要定义为 mutable closure
///
/// > 闭包的 borrow 也遵循 rust 的 borrow 规则
///
/// > 当闭包 borrow 一个值时，实现了 Copy trait 的会将自己 copy 传给闭包
///   但是不可复制的类型将 move 到闭包内
///
/// > 在闭包 `||` 之前使用 move 关键字会将闭包捕获的变量所有权强制转移到闭包内

fn main() {
    fn function(num: u32) -> u32 {
        num + 1
    }

    let closure_annotated = |num: u32| num + 1;

    let num = 1u32;

    println!("function execute: {}", function(num));
    println!("closure execute: {}", closure_annotated(num));

    let return1 = || 1;
    println!("closure just return 1 execute: {}", return1());

    // 一、capturing

    let color = String::from("red");
    let print = || println!("color: {}", color);
    print();

    let color_ref = &color;
    println!("color reference: {}", color_ref);

    // color 可再次被 borrow
    let _color_reborrow = color;

    // mut closure
    // 当需要在闭包内改变一个变量时，我们需要用到可变闭包
    // 1. 不在闭包内修改一下变量，需要用到它的 owner or mutable reference
    // 2. 由于 mutable reference 本身不够严谨，故而最好直接 borrow
    let mut count = 0;
    // 当 borrow count 发生了改变，相对应的 incr closure 内部也发生了改变，所以需要使用 mutable 限定
    let mut incr = || {
        count += 1;
        println!("closure execute inc: {}", count);
    };
    incr();
    // 闭包 borrow 当持有引用时不能 borrow
    // let _count_reborrow = &count;
    incr();

    // 当闭包不再 borrow count 后，count 变得可借用
    let _mut_reborrow_count = &mut count;

    // 实现了 Copy trait 的会 copy 一份值放入闭包中，不会影响本身，未实现的将会 move 进闭包

    let movable = Box::new(11);

    let consume = || {
        println!("`movable: {}`", movable);
        // Box 本身不可复制，所以 movable move 到了闭包内，将其销毁后，后续继续使用 movable 会报错
        std::mem::drop(movable);
    };
    consume();
    // 此时 movable 已经 move 到 consume 内了
    // consume();

    // 添加 move 关键字，将所有权强制转移到闭包内
    let mut haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // move 到 contains 内了
    // let mut_haystack = haystack.as_mut_slice();
}
