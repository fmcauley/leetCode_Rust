
pub fn find_sum_index(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prior_index = 0;
    let mut prior = 0;
    let mut answer : Vec<i32> = vec!();

    for (index, value) in nums.into_iter().enumerate() {
        if target - value == prior {
           answer.push(prior_index);
           answer.push(index.try_into().unwrap());
           break;
        }
        prior = value;
        prior_index = index as i32;
    
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_a_vector_and_a_target_find_index() {
        let nums = vec!(1,2);
        let target = 3;
        let output = find_sum_index(nums,target);
        assert_eq!(output, vec!(0,1));
    }

    #[test]
    fn given_a_new_vector_and_a_target_find_index() {
        let nums = vec!(1,2,4,5,6,7);
        let target = 11;
        let output = find_sum_index(nums,target);
        assert_eq!(output, vec!(3,4));
    }

}