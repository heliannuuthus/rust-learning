use std::ops::{Add, Mul, Sub};
/// ## DRY （Don't Repeat Yourself）
///
///
///
///

// 比较两个 Vec 的长度
macro_rules! assert_eq_len {
    // tt token tree 标识符，表示运算符或者标识
    ($a:ident, $b:ident, $fn:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: nomatching length {:?} {:?} {:?}",
            stringify!($fn),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func_name:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func_name<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_eq_len!(xs, ys, $func_name, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(sub_assign, Sub, -=, sub);
op!(mul_assign, Mul, *=, mul);

mod test {
    macro_rules! test_all_assign {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = std::iter::repeat($x).take(size).collect();
                    let y: Vec<_> = std::iter::repeat($y).take(size).collect();
                    let z: Vec<_> = std::iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z)
                }
            }
        };
    }
    test_all_assign!(add_assign, 1u32, 2u32, 3u32);
    test_all_assign!(sub_assign, 3u32, 2u32, 1u32);
    test_all_assign!(mul_assign, 5u32, 2u32, 10u32);
}

fn main() {
    let mut aa: Vec<u32> = vec![1u32, 2u32, 3u32, 4u32];
    let aa1 = vec![3u32, 4u32, 5u32, 6u32];
    add_assign(&mut aa, &aa1);
    println!("add cc: {:?}", aa);

    let mut ss: Vec<u32> = vec![3u32, 4u32, 5u32, 6u32];
    let ss1 = vec![1u32, 2u32, 3u32, 4u32];
    sub_assign(&mut ss, &ss1);
    println!("sub ss: {:?}", ss);

    let mut mm: Vec<u32> = vec![1u32, 2u32, 3u32, 4u32];
    let mm1 = vec![3u32, 4u32, 5u32, 6u32];
    mul_assign(&mut mm, &mm1);
    println!("mul mm: {:?}", mm);
}
