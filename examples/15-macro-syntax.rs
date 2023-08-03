///
/// ## 宏 macro
///
/// > Rust 中，宏不会执行函数方法，编译器处理宏仅仅是将其展开成代码，抽象语法树（abstract syntax tree）
///
/// ### 指示符
///
/// 宏由两部分组成
/// 
/// * 宏的参数使用一个 `$` 符号作为前缀，使用一个**指示符**注明类型
/// * 宏的代码块内 `() => ()` 作为被编译器展开后真正的代码
///
///     * `block`
///     * `expr` 表达式
///     * `ident` 变量名、函数名
///     * `item`
///     * `literal` 字面量
///     * `pat` pattern 模式匹配
///     * `path`
///     * `stmt` statement 语句
///     * `tt` token tree 标记树
///     * `ty` type 类型
///     * `vis` 可见性描述符
///
/// ### 重载
///
/// > 宏可以被重载，从而接受不同的参数组合，语法类似 match，每个分支用 `;` 表示结尾（最后一个可省略）
///
/// 重复
///
/// > 宏参数可以使用 `+` 来表示出现至少一次，用 `*` 表示缺省
///
/// * 通常使用 `$(...),+` 的形式 `...` 替换为表达式，可以匹配一个或多个表达式
///
///
macro_rules! say_hello {
    // () 表示不接受任何的参数
    () => {
        println!("hello")
    };
}
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("calling {:?}()", stringify!($func_name));
        }
    };
}
create_function!(foo);
create_function!(bar);

// 宏重载
macro_rules! test {
    ($left:expr;and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr;or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// 宏重复
macro_rules! find_min {
    // just return
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    );
}

fn main() {
    foo();
    bar();
    say_hello!();
    test!(1u32 + 1 == 2u32; and 2u32 * 2 == 4u32);
    test!(true; or false);
    println!("min: {}", find_min!(5u32, 2u32 * 3, 32u32));
}
