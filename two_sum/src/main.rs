use std::collections::HashMap;

fn greet(name: &String) {
    print!("Hello, {}!\n", name);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let required = target - num;
        match seen.get(num) {
            Some(&pair_index) => return vec![pair_index.clone(), index as i32],
            None => seen.insert(required, index as i32),
        };
    }
    return vec![-1, -1];
}

fn main() {
    let mut name = format!("Dhruv");
    name.push('X');
    let r = &name;
    greet(r);
    greet(r);
    name.push('X');
    let result = two_sum(vec![2,3,1,7,2,6,11,15], 9);
    println!("{}", result[0]);
    println!("{}", result[1]);
}
