mod bench;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 3;
    let y = 5;

    let (res, time) = bench::time_it(move || {
        add(x, y)
    });

    println!("res {:?}, time {:?}", res, time)
}
