fn main() {
    println!("Hello, world!");
    let mut a = 1;
    let mut nums = Vec::new();

    loop {
        println!("{}", a);
        if nums.len() >= 10 {
            nums = nums[1..].to_vec();
        }

        nums.push(a);

        println!("{:?}", nums);

        a += 1;
    }
}
