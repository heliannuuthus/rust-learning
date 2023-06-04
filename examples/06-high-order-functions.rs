/// ## 知识点
///
/// ### 高阶函数 （HOF high-order-function）
/// 
/// 高阶函数代表着可以使用一个或多个函数进行组合形成一个更有用的函数
///
///

fn main() {
    let is_odd = |n| n % 2 == 1;

    // 统计 upper 以内，平方是奇数的和
    // 上限
    let upper = 1000;
    let mut acc = 0;

    for num in 0.. {
        let num_squared = num * num;
        if num_squared >= upper {
            break;
        } else if is_odd(num_squared) {
            acc += num_squared;
        }
    }
    println!("squared odd sum: {:?}", acc);

    let squared_number = (0..)
        .map(|n| n * n)
        .take_while(|&n| n <= upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("squared odd sum: {:?}", squared_number);

}
