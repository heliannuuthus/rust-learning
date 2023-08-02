use std::fmt::Debug;

/// ## 生命周期（lifetime）
///
/// > 生命周期表示编译器（借用检查器）用它来保证所有的借用都是有效的
///
/// 确切的说保证的是每个借用在其出借者被销毁前，都应该是有效的，在其销毁后变成无效借用
///
/// 切记：作用域和生命周期不是同一个概念，往往，生命周期会和作用域一起提到
///
/// ### 显示标注
///
/// > 借用检查器通过显示标注的生命周期来 明确 借用应该被保持多久
///
/// * 生命周期可能会被省略
/// * 在没有省略的情况下，一定得显示的标注生命周期
///
///
/// ### 函数
///
/// 排除掉生命周期省略的情况
/// * 任何引用都必须标注好声明周期
/// * 任何返回值一定得和参数的生命周期相同或者是使用 ‘static
///
/// 注意：在没有入参的函数中，有可能会返回无效的借用，也就是返回的引用指向了无效的数据
///
/// ### 方法
///
/// 和函数一样
///
/// ### 结构体
///
/// 和函数以及方法一样
///
/// ### trait
///
/// > impl 也可以有生命周期。其他和函数一样
///
///
/// ### 约束
///
/// > 其实生命周期也可以使用约束（其本身就是泛型的一种）
///
/// * `+` 和泛型约束相同，都是类型的叠加
/// * `:` 和泛型的约束不太一样
///     * `T: 'a` 表示所有的声明周期都应该比 `'a` 长
///     * `T: Trait + 'a`: 表示所有类型都必须实现 `Trait` trait 且其中所有的引用生命周期都比 `'a` 长
///
/// ### 强制转换
///
/// > 一个较长生命周期的引用可以强制转换为一个较短的引用
///
/// 强制转换可以由编译器隐式的推导并执行，也可以通过声明不同的生命周期的形式出现
///
/// ### static
///
/// > `'static` 是可能的生命周期中最长的，伴随整个程序的结束才会终止
///
/// * `'static` 可以被转换成一个较短的生命周期
/// * 他们都把数据保存在可执行文件的只读区域
///     * 使用 `static` 关键字声明的常量
///     * 产生一个拥有 `&'static` str 的 String 字面量
///
///

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// 表示 t 得实现 `Debug` 这个 trait 并且生命周期要比 `'a` 长
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print`: t is {:?}", t);
}

// 传入两个生命周期不相同的借用，编译器可以隐式的转换为较短的一个，并且返回
fn multiple<'a>(s1: &'a u32, s2: &'a u32) -> u32 {
    s1 * s2
}

// `'a: 'b` 表示 `'a` 的声明周期至少比 `’b` 长，返回 `'a` 编译器不报错，这是强制转换的结果
fn choose_first<'a: 'b, 'b>(s1: &'a u32, _s2: &'b u32) -> &'b u32 {
    s1
}

const NUM: u32 = 19;

fn coerce_static<'a>(_s: &'a u32) -> &'a u32 {
    &NUM
}

fn main() {
    let x = 8;

    let reference = Ref(&x);
    print_ref(&reference);

    {
        let y = 9;
        println!("multiple: {:?}", multiple(&x, &y));
        println!("force choose_first: {:?}", choose_first(&x, &y))
    }
    //fn invalid_output<'a>() -> &'a String { &String::from("foo") }
    // 上面代码是无效的：`'a` 存活的时间必须比函数的长。
    // 这里的 `&String::from("foo")` 将会创建一个 `String` 类型，然后对它取引用。
    // 数据在离开作用域时删掉，返回一个指向无效数据的引用。

    {
        // 产生一个 string 并且打印他
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
        // 虽然离开了作用域后，static_string 引用不能再使用了，但是数据仍然在二进制文件内
    }
    {
        let test = 2;
        let ss = coerce_static(&test);
        println!("coerce_static: {}", ss);
    }
    println!("NUM cosntant: {}", NUM);

}
