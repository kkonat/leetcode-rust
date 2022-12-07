fn main() {
    println!("solution #1 Two Sums");
    let result = two_sum(vec![2, 7, 11, 15], 9);
    println!("{result:?}");
    assert_eq!(result, [0, 1]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let size = nums.len();
    for a in 0..size {
        for b in a + 1..size {
            if nums[a] == target - nums[b] {
                return vec![a as i32, b as i32];
            }
        }
    }
    return vec![0, 0];
}
