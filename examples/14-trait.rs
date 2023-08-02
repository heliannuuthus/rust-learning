/// ## 特质 Trait
///
/// > Trait 是对未知类型 Self 定义的方法集。Self 也能访问同一个 triat 中定义的其他方法
///
/// Rust 中所有的类型都可以实现 triat
///
/// ### 派生 derive
///
/// Rust 通过 #[derive] 提供了一些默认的 derive
///
/// * Compare：Eq, PartialEq, Ord, PartialOrd
/// * Clone：从 &T 复制得到一个 T
/// * Copy：获取一个“复制语义”的对象，而非一个“移动语义”
/// * Hash：计算 &T 的哈希值
/// * Default：提供空实例对象
/// * Debug：可以使用 `{:?}` formatter 格式化一个值
///
/// ### 使用 dyn返回 triat
///
/// > Rust 编译器需要知道每个方法的返回值所需要的空间大小，而一个 trait 的大小只能通过其实现对象决定，可以使用预分配静态堆内存来做到方法返回  trait
///
/// 如果使用这种方式返回 trait，那么必须加上 `dyn` 关键字
///
///
/// ### 运算符重载
///
///
/// > Rust 里的所有运算符都是通过调用相应的 trait 内约定的方法来实现的
///
/// 例如 `x + x` 就是实现了 `Add` trait 的对象，所以只要实现了 `Add` trait 的对象都能使用 `+` 运算符
///
/// ### Drop
///
/// > 当对象离开作用域时，会主动调用这个方法，`Drop` trait 的主要意义是释放实现者占有的实例资源
///
/// ### Iterator
///
/// > `Iterator` trait 仅需实现一个 next 方法，返回下一个值
///
/// * for 结构语法会使用 `.into_iter()` 方法，将集合或数组主动转成 iterator
///
/// ### impl Trait
///
/// > Rust 考虑到当返回结果过于复杂的情况下可以使用 impl trait 的语法，使得返回值是一个实现了 trait 的对象。大大简化类型签名
///
/// * 当某些 Rust 类型无法写出的时候（闭包的未知类型）。在使用 impl trait 语法之前，这样是需要在堆上进行分配才能返回闭包。
///
/// * 还能使用 impl trait 返回 `map`、`filter` 等闭包的迭代器
///
/// ### Clone
///
/// > 在处理资源的时候，默认行为是在赋值或函数调用的时候同时将他们 move 进入作用域，但是有时候也需要将资源复制一份
///
/// ### 父 trait
///
/// > Rust 没有 extends，但是可以将 trait 定义为另一个的 trait 超集
///
/// ```rust
/// trait Person {}
///
/// trait Staudent : Person{}
/// 
/// trait Programmer {}
/// 
/// trait CompSciStudent: Programmer + Student {}
/// ```
/// 
/// # 消除重复的 trait
/// 
/// > 对象可以实现所有的 trait 如果有多个 trait 的方法名称相同，可以从单个 trait 的 `impl{}` 代码块内，使用的时候使用全限定语法调用
/// 
/// ```rust
///     <parent as child>::get()
/// ```
///  

// impl trait
fn make_adder_function(y: u32) -> impl Fn(u32) -> u32 {
    move |x: u32| x + y
}

fn return_closure_iter<'a>(numbers: &'a Vec<u32>) -> impl Iterator<Item = u32> + 'a {
    numbers.iter().map(|x| x * 2).filter(|x| x > &2)
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} say: {}", self.name(), self.noise())
    }
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name())
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // 默认 trait 方法可以重载。
    fn talk(&self) {
        // 例如我们可以增加一些安静的沉思。
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut sheep = Sheep::new("boy");
    sheep.talk();
    sheep.shear();
    sheep.talk();
    let tmp_vec = vec![1, 2, 3];
    let mut return_vec = return_closure_iter(&tmp_vec);
    if let Some(good) = return_vec.next() {
        print!("{} ", good);
        for good in return_vec {
            print!("{} ", good);
        }
    };
}
