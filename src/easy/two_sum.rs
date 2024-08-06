pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result:Vec<i32> = vec![];
    let length = nums.len();

    for x in 0 .. length-1 {
        for y in x+1 .. length {
            if nums[x] + nums[y] == target {
                result.push(x.try_into().unwrap());
                result.push((y).try_into().unwrap());
            }
        }
    }

    return result;
}

// ? Se puede mejorar con hashmap 