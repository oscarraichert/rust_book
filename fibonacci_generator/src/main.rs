fn main() {
    let mut prev_num: i32;
    let mut current_num: i32 = 0;
    let mut next_num: i32 = 1;

    loop {
        println!("{}", next_num);
        prev_num = current_num;
        current_num = next_num;
        next_num = current_num + prev_num;
    }
}
