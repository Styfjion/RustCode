use std::collections::HashMap;

pub struct Solution {

}

impl Solution {

    fn calculate_delta(num: i32) -> i32 {
        let mut reverse_num = 0;
        let mut temp = num;
        while temp > 0 {
            let digit = temp % 10;
            temp /= 10;
            reverse_num *= 10;
            reverse_num += digit;
        }
        num - reverse_num
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt: i64 = 0;
        const MOD: i64 = 1000000007;
        let mut records: HashMap<i32, i64> = HashMap::new();
        for num in nums {
            let delta = Solution::calculate_delta(num);
            *records.entry(delta).or_insert(0) += 1;
        }

        for (_, value) in records {
            cnt += value * (value - 1) / 2;
        }
        return (cnt % MOD) as i32
    }
}