/// ##知识点
/// > rust 没有原生类型的 隐式转换，例如， i32 -> f32, 但是可以使用 as 将 i32 转换为 f32
/// 
/// > 在将所有类型转换为无符号类型 T 时，对于超出范围的值，会一直加上或者减去 std::T::MAX + 1，直到满足 T 类型所在范围（其中 T 代表转化的目标类型）

fn main() {
  let decimal = 222.323;
  // 转换错误
  // let integer: u8 = decimal;
  // 使用 as 将 f64 -> u8
  let integer: u8 = decimal as u8;
  // 还能将 u8 -> char
  let character: char = integer as char;
  println!(
      "casting: f64({}) -> u8({}) -> char({})",
      decimal, integer, character
  );
  println!("1000 as a u16 is: {}", 1000 as u16);
  println!("1000 as a u8 is: {}",  0b11101000);
  println!("  -1 as a u8 is : {}", (-1i8) as u8);
}