/// ## 路径和文件
/// 
/// prelude 会选择并输出符合平台类型的 Path（表示 Rust 在每个程序导入的一些通用的东西）
/// 
/// ### Path
/// 
/// > Path 分为 `posix::Path` 针对 Unix 系统，以及 `windows::Path` 针对 windows 系统
/// 
/// * Path 底层的数据结构是 Vec<u8> 字节数组，所以将其强制转换为 &str 并不是无开销的，并且也可能会失败，所以返回 Option
/// 
/// 
/// ### File I/O
/// 
/// > File 结构体表示一个被打开的文件
/// 
/// * 在进行文件操作时随时有可能发生错误，所以 File 的所有方法都返回 std::io::Result
/// 
/// 
/// 

fn main() {
   
}
