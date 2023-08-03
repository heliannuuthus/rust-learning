use std::{error::Error, fmt::Display, num::ParseIntError, result, vec};

/// ## 异常处理
///
///
///
///

// step1. 出现两种错误
// fn double_first(vec: Vec<&str>) -> u32 {
//     let first = vec.first().unwrap(); // erorr 1
//     2 * first.parse::<u32>().unwrap() // error 2
// }

// step2. 将返回值用 Option or Result 表示处理 error1
// fn double_first(vec: Vec<&str>) -> Option<Result<u32, ParseIntError>> {
//     let first = vec
//         .first()
//         .map(|first| first.parse::<u32>().map(|x| 2u32 * x));
//     first
// }

// step3. 将返回值使用 Result 替代，这样可以直接使用 `?` 拆箱 Result，如果 Option 为 None 直接可以处理 erorr
// fn double_first(vec: Vec<&str>) -> Result<Option<u32>, ParseIntError> {
//     vec.first()
//         .map(|first| first.parse::<u32>().map(|x| 2u32 * x))
//         .map_or(Ok(None), |x| x.map(Some))
// }

// step4 将 Error 都转换为同一个最顶级的 Erorr 会简化代码
// * 一种类型对应了多种错误
// * 向用户提供清楚的错误信息
// * 容易区分错误类型
//    * goodjob: Err(msg, error)
//    * badjob: Err(msg)
// * 能够容纳错误信息
//    * goodjob: Err(msg, position)
//    * badjob: Err(msg)
// * 能够与其他的错误很好结合

// type Result<T> = std::result::Result<T, DoubleError>;

// #[derive(Debug)]
// struct DoubleError;

// // step4.1 保证 DoubleError 可以输出结果
// impl Display for DoubleError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }

// // step4.2 让 DoubleError 可以被 std::error:Error 包裹
// impl Error for DoubleError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }

// // step4.3 处理 Erorr 统一输出类型
// fn double_first(vec: Vec<&str>) -> Result<u32> {
//     // 使用 and_then 的意义在于 map_err 最后会返回一个 Result<T, Err> 所以最后得到的是一个 Result<Result<T, Err>, Err>
//     vec.first()
//         .ok_or(DoubleError)
//         .and_then(|x| x.parse::<u32>().map_err(|_| DoubleError).map(|g| g * 2))
// }

// step5 将 Erorr 装箱处理,不过这样就没法静态识别 Erorr 只能在运行时区分 Error

// > 对任何实现了 std:error::Erorr trait 的对象,标准库的 `Box` 都提供 `From` 方法让其装箱

// step5.1 定义一个可以接受 Error trait 的 Result Result

// type Result<T> = result::Result<T, Box<dyn Error>>;

// // step5.2 定义一个 Error 实现 std::erorr::Erorr

// #[derive(Debug)]
// struct EmptyVec;

// impl Display for EmptyVec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Vec is empty")
//     }
// }

// impl Error for EmptyVec {
//     fn description(&self) -> &str {
//         "invalid first item to double"
//     }
//     fn cause(&self) -> Option<&dyn Error> {
//         // 泛型错误。没有记录其内部原因。
//         None
//     }
// }

// // step5.3 处理 Erorr
// fn double_first(vec: Vec<&str>) -> Result<u32> {
//     // 使用 and_then 的意义在于 map_err 最后会返回一个 Result<T, Err> 所以最后得到的是一个 Result<Result<T, Err>, Err>
//     vec.first()
//         .ok_or(EmptyVec.into())
//         .and_then(|x| x.parse::<u32>().map_err(|e| e.into()).map(|g| g * 2))
// }

// step6 使用 `?` 实现 unwrap 和 return Err(From::from(err)) 自动装箱

// type Result<T> = result::Result<T, Box<dyn Error>>;

// #[derive(Debug)]
// struct EmptyVec;

// impl Display for EmptyVec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Vec is empty")
//     }
// }

// impl Error for EmptyVec {}

// // step6.2 使用 `?` 使得 double_first 对 Error 自动装箱

// fn double_first(vec: Vec<&str>) -> Result<u32> {
//     let first = vec.first().ok_or(EmptyVec)?;
//     Ok(first.parse::<u32>().map(|x| x * 2)?)
// }

// step7 使用 enmu 进行包装

type Result<T> = result::Result<T, DoubleError>;
#[derive(Debug)]
enum DoubleError {
    VecEmpty,
    Parse(ParseIntError),
}
// step7.1 让 DoubleError 可以被 std::error:Error 包裹
impl Display for DoubleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match *self {
        DoubleError::VecEmpty => write!(f, "empty vec"),
        DoubleError::Parse( ref e) => write!(f, "invalid first double  "),
      }
        
    }
}

impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DoubleError::VecEmpty => None,
            DoubleError::Parse( ref e) => Some(e),
        }
    }
}

// 7.2 使 ParseIntErorr 可以用转换为 DoubleError
impl From<ParseIntError> for DoubleError {
    fn from(value: ParseIntError) -> Self {
        DoubleError::Parse(value)
    }
}

// 7.3 double_first
fn double_first(vec: Vec<&str>) -> Result<u32> {
    let first = vec.first().ok_or(DoubleError::VecEmpty)?;
    Ok(first.parse::<u32>().map(|x| x * 2)?)
}

fn print(res: Result<u32>) {
    match res {
        Ok(r) => println!("double is: {:?}", r),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["opt", "2", "23"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
