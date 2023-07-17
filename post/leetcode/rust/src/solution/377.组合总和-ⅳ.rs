/*
 * @lc app=leetcode.cn id=377 lang=rust
 *
 * [377] 组合总和 Ⅳ
 *
 * https://leetcode-cn.com/problems/combination-sum-iv/description/
 *
 * algorithms
 * Medium (42.92%)
 * Likes:    230
 * Dislikes: 0
 * Total Accepted:    19K
 * Total Submissions: 44.2K
 * Testcase Example:  '[1,2,3]\n4'
 *
 * 给定一个由正整数组成且不存在重复数字的数组，找出和为给定目标正整数的组合的个数。
 *
 * 示例:
 *
 *
 * nums = [1, 2, 3]
 * target = 4
 *
 * 所有可能的组合为：
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 *
 * 请注意，顺序不同的序列被视作不同的组合。
 *
 * 因此输出为 7。
 *
 *
 * 进阶：
 * 如果给定的数组中含有负数会怎么样？
 * 问题会产生什么变化？
 * 我们需要在题目中添加什么限制来允许负数的出现？
 *
 * 致谢：
 * 特别感谢 @pbrother 添加此问题并创建所有测试用例。
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[t]: 表示目标数为 t 的组合总数；
    /// 2. 则对于nums中的任何一个n, dp[t-n]表示目标为t-n的组合总数；
    /// 3. 有 dp[t] = sum(dp[t-n]) (n in nums, 且n<=t)
    /// 4. 初始条件: dp[0] = 1, (当目标为0时, )
    /// 5. 终止条件: dp[target]
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        let target = target as usize;
        for t in 1..=target {
            for &n in &nums {
                if n as usize <= t {
                    dp[t] += dp[(t - n as usize)];
                }
            }
        }

        dp[target]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }
}
