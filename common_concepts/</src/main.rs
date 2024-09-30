fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    exp();
    print_five();
    plus_main(); 
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn exp() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn print_five() {
    let xfive = five();

    println!("The value of xfive is: {xfive}");
}

fn plus_main() {
    let plusx = plus_one(5);

    println!("The value of plus is: {plusx}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // I’m feeling lucky today
}

// hello, world

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.