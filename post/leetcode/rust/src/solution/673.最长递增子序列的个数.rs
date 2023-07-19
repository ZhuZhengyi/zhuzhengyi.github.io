/*
 * @lc app=leetcode.cn id=673 lang=rust
 *
 * [673] 最长递增子序列的个数
 *
 * https://leetcode.cn/problems/number-of-longest-increasing-subsequence/description/
 *
 * algorithms
 * Medium (44.64%)
 * Likes:    766
 * Dislikes: 0
 * Total Accepted:    88.1K
 * Total Submissions: 197.4K
 * Testcase Example:  '[1,3,5,4,7]'
 *
 * 给定一个未排序的整数数组 nums ， 返回最长递增子序列的个数 。
 *
 * 注意 这个数列必须是 严格 递增的。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: [1,3,5,4,7]
 * 输出: 2
 * 解释: 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
 *
 *
 * 示例 2:
 *
 *
 * 输入: [2,2,2,2,2]
 * 输出: 5
 * 解释: 最长递增子序列的长度是1，并且存在5个子序列的长度为1，因此输出5。
 *
 *
 *
 *
 * 提示:
 *
 *
 *
 *
 * 1 <= nums.length <= 2000
 * -10^6 <= nums[i] <= 10^6
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 length[i]: nums[0..i]序列的最长递增子序列;
    ///       count[i]: nums[0..i]最长递增子序列的个数;
    /// 2. lenght[i] = max(length[j]) + 1 (0=<j<i, nums[i]>nums[j])
    ///    count[i] =
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut length = vec![1; nums.len()];
        let mut count = vec![1; nums.len()];
        let mut max_length = 1;
        for i in 1..nums.len() {
            for j in 0..i {
                // 遇到递增的两个数
                if nums[j] < nums[i] {
                    // 更新length[i], 为max(length[j]) + 1
                    if length[i] < length[j] + 1 {
                        length[i] = length[j] + 1; 
                        count[i] = count[j]; // 记录最新的count[i]为对应的count[j]
                    } else if length[i] == length[j] + 1 {
                        // 找到同样长度的递增子序列
                        count[i] += count[j]; // 合并count[i]
                    }
                }
            }
            if length[i] > max_length {
                // 更新max_length
                max_length = length[i];
                res = count[i];
            } else if length[i] == max_length {
                //统计相同长度的最长递增子序列次数和
                res += count[i];
            }
        }

        res
    }
}
// @lc code=end
struct Solution;
