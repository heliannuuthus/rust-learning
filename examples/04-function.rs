// 方法： 依赖对象的函数，可使用 self 访问其内部的数据，在 rust 中使用 impl 定义一个实力方法代码块

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((y2 - y1) * (x2 - x1)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((y2 - y1).abs() + (x2 - x1).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        // 获取 self owner 所有权，解析赋值拿到堆上数据
        let Pair(first, second) = self;
        println!("destroy pair({:?}, {:?})", first, second);
        // 出了作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(2.2, 3.3),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    // 必须是 mutable
    // rectangle.translate(3.0, 4.0);

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(4.0, 4.0),
    };
    square.translate(5.0, 5.0);
    // 平移不变
    println!("square perimeter: {}", square.perimeter());
    println!("Square area: {}", square.area());

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // 传入 owner 无法继续使用，编译错误
    // pair.destroy();
}
