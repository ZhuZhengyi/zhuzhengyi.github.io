/*
 * @lc app=leetcode.cn id=1218 lang=rust
 *
 * [1218] 最长定差子序列
 *
 * https://leetcode.cn/problems/longest-arithmetic-subsequence-of-given-difference/description/
 *
 * algorithms
 * Medium (51.71%)
 * Likes:    225
 * Dislikes: 0
 * Total Accepted:    41.8K
 * Total Submissions: 80.7K
 * Testcase Example:  '[1,2,3,4]\n1'
 *
 * 给你一个整数数组 arr 和一个整数 difference，请你找出并返回 arr 中最长等差子序列的长度，该子序列中相邻元素之间的差等于
 * difference 。
 * 
 * 子序列 是指在不改变其余元素顺序的情况下，通过删除一些元素或不删除任何元素而从 arr 派生出来的序列。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：arr = [1,2,3,4], difference = 1
 * 输出：4
 * 解释：最长的等差子序列是 [1,2,3,4]。
 * 
 * 示例 2：
 * 
 * 
 * 输入：arr = [1,3,5,7], difference = 1
 * 输出：1
 * 解释：最长的等差子序列是任意单个元素。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：arr = [1,5,7,8,5,3,4,2,1], difference = -2
 * 输出：4
 * 解释：最长的等差子序列是 [7,5,3,1]。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * -10^4 
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 动态规划
    /// 1. 令 f[n]: 以difference为定差,以n为结尾的最长子序列长度；
    /// 2. 对于n - difference，满足：
    ///         f[n] = f[n-difference] + 1
    /// 3. 最终问题答案为:  max(f[n]), (n=arr[0]..arr[n])
    ///       
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        use std::collections::HashMap;

        let mut dp = HashMap::with_capacity(4096);

        arr.iter()
            .map(|&n|{
                let d = dp.get(&(n-difference)).unwrap_or(&0) + 1;
                dp.insert(n, d);
                d
            })
            .max()
            .unwrap_or_default()
    }
}
// @lc code=end

