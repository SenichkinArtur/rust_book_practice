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
    while_loop();
    for_loop();
    for_range_loop();
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

fn while_loop() {
    let mut count = 3;

    while count != 0 {
        println!("count is {count}");

        count -= 1;
    }

    println!("while loop finished");
}

fn for_loop() {
    let array = [1, 2, 3, 4, 5];

    for item in array {
        println!("items is {item}");
    }
}

fn for_range_loop() {
    for item in (1..5).rev() {
        println!("item is {item}");
    }
}
