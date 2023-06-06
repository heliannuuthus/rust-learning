use std::{
    fmt::{Debug, Display},
    vec,
};

/// ## 知识点
/// ### 约束（bounds）
///
/// > 当使用范型时通常会将类型参数指定为某一个 trait，当指定为这个 trait 之后我们可以称之为对该范型有一个 **约束**
///
///
/// 约束通常会限定泛型范围为某一种多种符合约束的类型
///
/// 约束的另一种作用是泛型的实例可以访问作为约束的 trait 定义的方法
///
/// 简单的说，传入实现了某一种 trait 的泛型类型，编译器可以在当前方法内直接调用该 trait 的方法，属于是提前约束并知道调用的方法
///
/// > rust 内存在空约束，该种约束也是拿来做类型约束（main），不包含任何的 function
///
///
/// > 多重约束：可以使用 `+` 链接多个 trait 达到 **既xxx又xxx** 的目的
///
/// > 需要定义多个泛型类型，对应多个约束时应当使用 `,` 分割
///
/// > where 子句：可以用 where 从句定义类型参数的约束。当然 where 可以作用于所有的参数类型，不局限于对类型参数的约束
///
/// > 

// 此时调用 printer 方法的必须是实现了 Display trait 的对象
fn printer<T: Display>(t: T) {
    println!("{}", t)
}

struct S<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

fn print_debug<T: Debug>(content: &T) {
    println!("print_debug: {:?}", content);
}

fn area<T: HasArea>(has_area: &T) -> f64 {
    has_area.area()
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    heigh: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        return self.heigh * self.width;
    }
}

#[allow(dead_code)]
struct Triangle {
    width: f64,
    height: f64,
}

trait PrintInOption {
    fn print_in_option(self);
}

// impl<T: Debug> PrintInOption for Option<T> {
//     fn print_in_option(self) {
//         println!("{:?}", Some(self))
//     }
// }

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

/// ## 第二种写法
/// ```rust
/// impl<T> PrintInOption for T where
///     Option<T>: Debug {
///     fn print_in_option(self) {
///         println!("{:?}", Some(self));
///     }
/// }
/// ```

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
    // Vec 没有实现 Display trait 所以传入 S(vec!()) 编译不通过
    // let case = vec![123, 1];
    // let s = S(case);
    // Vec 没有实现 Display trait，传入 printer() function 内编译不通过
    // printer(case)

    let t = Triangle {
        width: 2.2,
        height: 2.2,
    };
    let r = Rectangle {
        width: 3.3,
        heigh: 4.3,
    };

    // Triangle 没有实现 HasArea trait 所以不能作为 fn area 参数被调用
    // area(&t);
    // Rectangle 实现了 HasArea trait
    print!("area: {}", area(&r));

    print_debug(&r);
}
