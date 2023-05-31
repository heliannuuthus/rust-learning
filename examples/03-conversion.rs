/// ## 知识点
///
/// ### From and Into
/// From 和 Into 是两个相从内部关联的 trait，如果能从类型 B —> A 那么也就能通过 A -> B
/// > 注意，Into 往往需要指定赋值变量的类型
///
/// ### TryFrom and TryInto
/// TryFrom 和 TryInfo 和 From Into 类似，唯一一点不相同的地方在于 TryFrom 和 TryInfo 往往出现在转换容易错误的部分，所以他们的返回值是类型 Result
///
/// ### ToString and FromStr
/// 要把任何类型转换成 String 仅需要实现 ToString 这个 trait，但是最好是直接实现 Display trait 他不仅是 ToString 的 parent trait 也提供了 Println 宏内的打印
/// 当然有很多时候也会将 String 转换成其他不同的类型，此时需要实现 FromStr 这个 trait，调用 parse::<T> 方法就行
use std::{
  convert::{From, TryFrom},
  fmt::Display,
  str::FromStr,
};
#[derive(Debug)]
struct TNumber {
  value: u32,
}

impl From<u32> for TNumber {
  fn from(value: u32) -> Self {
      TNumber { value: value }
  }
}

#[derive(Debug, PartialEq)]
struct TEventNumber(u32);

impl TryFrom<u32> for TEventNumber {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
      if value % 2 == 0 {
          Ok(TEventNumber(value))
      } else {
          Err(())
      }
  }
}

struct TString {
  value: u32,
}

impl Display for TString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "define myself TString struct: {}", self.value)
  }
}

impl FromStr for TString {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let va = s.parse::<u32>().unwrap();
      Ok(TString { value: va })
  }
}

fn main() {
  // From and Into
  let tn = TNumber::from(32);
  println!("TNumber: {:?}", &tn.value);
  println!("TNumber: {:?}", tn.value);
  println!("TNumber: {:?}", tn);

  let tn1: TNumber = 52u32.into();
  println!("TNumber1: {:?}", &tn1.value);
  println!("TNumber1: {:?}", tn1.value);
  println!("TNumber1: {:?}", tn1);

  // TryFrom and TryInto

  assert_eq!(TEventNumber::try_from(6u32), Ok(TEventNumber(6u32)));
  assert_eq!(TEventNumber::try_from(5), Err(()));

  let result8: Result<TEventNumber, ()> = 8u32.try_into();
  assert_eq!(result8, Ok(TEventNumber(8)));
  let result7: Result<TEventNumber, ()> = 7u32.try_into();
  assert_eq!(result7, Err(()));

  // ToString and FromStr

  let ts = TString { value: 32 };

  println!("look at TString display: {}", ts.to_string());
  println!(
      "look at TString display: {}",
      "221".parse::<TString>().unwrap()
  );
}
