/*
 * @lc app=leetcode.cn id=1027 lang=rust
 *
 * [1027] 最长等差数列
 *
 * https://leetcode-cn.com/problems/longest-arithmetic-subsequence/description/
 *
 * algorithms
 * Medium (45.06%)
 * Likes:    80
 * Dislikes: 0
 * Total Accepted:    7.4K
 * Total Submissions: 16.4K
 * Testcase Example:  '[3,6,9,12]'
 *
 * 给定一个整数数组 A，返回 A 中最长等差子序列的长度。
 * 
 * 回想一下，A 的子序列是列表 A[i_1], A[i_2], ..., A[i_k] 其中 0 <= i_1 < i_2 < ... < i_k <=
 * A.length - 1。并且如果 B[i+1] - B[i]( 0 <= i < B.length - 1) 的值都相同，那么序列 B
 * 是等差的。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：[3,6,9,12]
 * 输出：4
 * 解释： 
 * 整个数组是公差为 3 的等差数列。
 * 
 * 
 * 示例 2：
 * 
 * 输入：[9,4,7,2,10]
 * 输出：3
 * 解释：
 * 最长的等差子序列是 [4,7,10]。
 * 
 * 
 * 示例 3：
 * 
 * 输入：[20,1,15,3,10,5,8]
 * 输出：4
 * 解释：
 * 最长的等差子序列是 [20,15,10,5]。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= A.length <= 2000
 * 0 <= A[i] <= 10000
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设dp[i][d]: 以a[i]为尾d为公差的最长等差序列长度;
    /// 2. 题目转化为: max(dp[i][d])  (i=0..n, d=-10000..10000);
    /// 3. 初始边界: dp[0][] = 1;
    /// 4. 递推公式: dp[i][d] = dp[j][d] + 1 (nums[i] = nums[j] + d)
    /// 5. 由递推公式, d=nums[i]-nums[j], 为保证d作为数组下标在合理范围内,
    ///    将d加上nums中最大最小值的差值, 由于 0<=nums[i]<=500, 所以+500;
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![1; 2*500 + 1]; a.len()];
        for i in 1..a.len() {
            for j in 0..i {
                let d = (a[i] - a[j] + 500) as usize;
                dp[i][d] = dp[j][d] + 1;
                res = res.max(dp[i][d]);
            }
        }

        res
    }
}
// @lc code=end

