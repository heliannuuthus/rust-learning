use std::thread::spawn;

/// ### 线程
fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    for (i, element) in data.split_whitespace().enumerate() {
        children.push(spawn(move || -> u32 {
            let res = element
                .chars()
                .map(|x| x.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, res);
            res
        }))
    }

    let mut intermediate = vec![];
    for child in children {
        intermediate.push(child.join().unwrap());
    }
    println!("sum: {}", intermediate.iter().sum::<u32>());
}
