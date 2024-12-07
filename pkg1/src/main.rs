fn main() {
    // let tuple = (420, 96);
    // let (x, y) = tuple;
    let addxy = sum(2, -1);
    println!("|| sum: {} ||", addxy);

    if addxy < 10 {
        println!("sum less than 10");
    } else if addxy == 10 {
        println!("summ equal to 10");
    } else {
        println!("sum more than 10");
    }

    let zero_if_terminal_else_one = if addxy == 1 {0} else {1};
    println!("zero_if_terminal_else_one: {}", zero_if_terminal_else_one);

    let mut counter = 0;
    let mut result = loop{
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("Result Counter after loop: {}", result);

    while result != 0 {
        result -= 1;
    }
    println!("Result Counter after while loop: {}", result);

    let arr = [1, 2, 3, 4, 5];

    for el in arr.iter() {
        println!("|| element : {} ||", el);
    }
}

fn sum(x:i32, y:i32) -> i32{
    println!("|| arg1: {} || arg2: {} ||", x, y);
    let sum = x + y;
    return sum;
}