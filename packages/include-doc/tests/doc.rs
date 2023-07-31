struct MyStruct;

fn setup_first_example(_: MyStruct) {}

fn setup_second_example(_: MyStruct) {}

pub fn my_first_example() {
    setup_first_example(MyStruct);
    println!("Hello, world!");
}

pub fn my_second_example() {
    setup_second_example(MyStruct);
    println!("Hello, world!");
}
