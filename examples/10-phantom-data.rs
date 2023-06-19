/// ## 知识点
///
/// ### phantom type （虚类型）
///
/// > 一种在运行时不存在，仅在编译时做静态检查的类型
///
/// > 虚类型，类似 Java ? 类型，仅仅只是一个占位符，通常用于定义某个 struct 的时候使用
///
///
///
use std::{marker::PhantomData, ops::Add};
#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let len1: Length<Inch> = Length(32.2, PhantomData);
    let len2: Length<Mm> = Length(32.1, PhantomData);
    println!("length 3add result: {:?}", (len1 + len1).0);
    println!("length 2 add result: {:?}", (len2 + len2).0);
}
