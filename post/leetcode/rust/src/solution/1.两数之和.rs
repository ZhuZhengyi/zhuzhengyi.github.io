/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 *
 * https://leetcode-cn.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (47.11%)
 * Likes:    7621
 * Dislikes: 0
 * Total Accepted:    814.7K
 * Total Submissions: 1.7M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
 *
 * 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
 *
 * 示例:
 *
 * 给定 nums = [2, 7, 11, 15], target = 9
 *
 * 因为 nums[0] + nums[1] = 2 + 7 = 9
 * 所以返回 [0, 1]
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - hashmap
    /// 1. 遍历序列；
    /// 2. 遍历过程中，使用hashmap记录遍历过的序列;
    /// 3. 如果遇到和遍历过的序列和为target的元素，则找到，返回true;
    /// 4. 否则不存在, 返回false；
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            match map.get(n) {
                Some(&pre_idx) => {
                    return vec![pre_idx, i as i32];
                }
                None => {
                    map.insert(target - n, i as i32);
                }
            }
        }
        vec![]
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum([2, 7, 11, 15].into(), 9), [0, 1]);
        assert_eq!(Solution::two_sum([3, 2, 4].into(), 6), [1, 2]);
        assert_eq!(Solution::two_sum([3, 3].into(), 6), [0, 1]);
    }
}
