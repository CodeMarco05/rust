

fn main() {

    let nums = vec![2,7,11,15];
    let target = 9;

    let result = two_sum(nums, target);

    for ele in result {
        println!("{}", ele)
    }

}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    /*for (index, &num) in nums.iter().enumerate(){

    }*/
    'outer: for i in 0..nums.len(){
        for j in i+1..nums.len(){
            if nums[i] + nums[j] == target {
                result.push(i as i32);
                result.push(j as i32);
                break 'outer;
            }
        }
    }

    return result;
}
