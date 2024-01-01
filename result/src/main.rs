use std::fs::File;

fn main() {
    let _greeting_file_result =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
