
pub fn find_sum_index(nums: Vec<i32>, target: i32) -> Vec<i32> {
    return vec!(0);
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

}