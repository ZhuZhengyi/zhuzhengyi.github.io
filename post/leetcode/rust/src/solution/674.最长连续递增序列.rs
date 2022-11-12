/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 *
 * https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence/description/
 *
 * algorithms
 * Easy (49.78%)
 * Likes:    218
 * Dislikes: 0
 * Total Accepted:    96.7K
 * Total Submissions: 193.3K
 * Testcase Example:  '[1,3,5,4,7]'
 *
 * 给定一个未经排序的整数数组，找到最长且 连续递增的子序列，并返回该序列的长度。
 * 
 * 连续递增的子序列 可以由两个下标 l 和 r（l < r）确定，如果对于每个 l ，都有 nums[i] < nums[i + 1] ，那么子序列
 * [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] 就是连续递增子序列。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,3,5,4,7]
 * 输出：3
 * 解释：最长连续递增序列是 [1,3,5], 长度为3。
 * 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为 5 和 7 在原数组里被 4 隔开。 
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [2,2,2,2,2]
 * 输出：1
 * 解释：最长连续递增序列是 [2], 长度为1。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * -10^9 
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 贪心法
    /// 1. 设 di 表示 以nums[i]为尾元素的递增序列长度；
    /// 2. 从左至右依次遍历nums，比较当前num和上一个num prev的大小；
    /// 3. 如果当前num>prev, 则将di + 1；
    /// 4. 否则num<=prev, 则di = 1；
    /// 5. 返回所有 max(di)
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut prev: i32 = -10^9;
        let mut d = 0;
        nums.iter()
            .map(|&n|{
                d = if n > prev { d + 1 } 
                    else { 1 } ;
                prev = n;
                d
            })
            .max()
            .unwrap_or_default()
    }
}
// @lc code=end

