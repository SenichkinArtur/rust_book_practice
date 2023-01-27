fn main() {
    let mut count: i32 = 0;

    let result = loop {
        if count == 10 {
            break count * 2;
        }

        count += 1;
    };

    println!("result is {result}");

    nested_loops();
}

fn nested_loops() {
    let mut count: i32 = 10;
    let mut inner_count: i32 = 2;

    'outer_loop: loop {
        if count == 0 {
            break 'outer_loop;
        }

        println!("count is {count}");

        'inner_loop: loop {
            println!("inner loop count: {inner_count}");

            inner_count -= 1;

            if inner_count == 0 {
                inner_count = 2;
                break 'inner_loop;
            }
        }

        count -= 1;
    }

    println!("loops finished");
}
