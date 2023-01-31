use include_doc_example::doc_with_tests::my_function;

struct MyFirstStruct;
struct MySecondStruct;

fn setup_first_example(_: MyFirstStruct) {}

fn setup_second_example(_: MySecondStruct) {}

pub fn my_first_example() {
    setup_first_example(MyFirstStruct);
    my_function();
}

pub fn my_second_example() {
    setup_second_example(MySecondStruct);
    my_function();
}
