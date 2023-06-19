/// ## 知识点
///
/// > 关联类型：解决了当存在场景，trait 使用多个类型需要由某一个类型决定时仅用声明该种类型即可
///
///
///
///

// new type
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// 已知问题，由于需要用到 Contains，Contains 定义了 A，B 两种类型，书写太过于复杂

fn difference<A, B, C>(contians: &C) -> i32
where
    C: Contains<A, B>,
{
    contians.last() - contians.first()
}

// 关联类型解决问题
trait Contains1 {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

fn difference1<C>(contains1: &C) -> i32
where
    C: Contains1,
{
    contains1.last() - contains1.first()
}
impl Contains1 for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn main() {
    let con: Container = Container(1, 3);
    println!("difference: {}", difference(&con));
    println!("difference1: {}", difference1(&con));
}
