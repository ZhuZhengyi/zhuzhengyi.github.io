/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 *
 * https://leetcode-cn.com/problems/longest-increasing-subsequence/description/
 *
 * algorithms
 * Medium (51.75%)
 * Likes:    1963
 * Dislikes: 0
 * Total Accepted:    371.6K
 * Total Submissions: 717.9K
 * Testcase Example:  '[10,9,2,5,3,7,101,18]'
 *
 * 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
 *
 * 子序列是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7]
 * 的子序列。
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [10,9,2,5,3,7,101,18]
 * 输出：4
 * 解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1,0,3,2,3]
 * 输出：4
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,7,7,7,7,7,7]
 * 输出：1
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
 *
 *
 * 进阶：
 *
 *
 * 你可以设计时间复杂度为 O(n^2) 的解决方案吗？
 * 你能将算法的时间复杂度降低到 O(n log(n)) 吗?
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设dp[i]: 表示以nums[i]为尾的最长递增序列的长度
    /// 2. 递推关系: dp[i] = max(dp[j]) + 1 (0=<j<i, nums[j]<nums[i])
    /// 3. 初始条件: dp[i] = 1 (i=0..n-1)
    /// 4. 目标: dp[nums.len()-1]
    pub fn length_of_lis0(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut res = 1;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            res = res.max(dp[i] as i32);
        }

        res
    }

    /// ## 解题思路2
    /// - 贪心+二分查找
    /// 1. 设 lis[i]: nums[0..i]的最长递增子序列;
    /// 2. 初始化 lis[0] = nums[0];
    /// 3. 从左至右, 依次遍历nums[1..]
    /// 4. 如果 nums[i] > lis.last(), 则将nums[i]加入到lis[]末尾;
    /// 5. 否则, nums[i] < lis.last(),
    ///    在lis[]中查找第一个大于nums[i]的数lis[j](0=<j<i), 将其替换为nums[i];
    /// 6. 由于lis[]为递增序列, 则可使用二分法快速查找lis[j]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lis = vec![]; //最长递增子序列
        for n in nums {
            match lis.last() {
                None => lis.push(n),
                Some(&l) if l < n => lis.push(n),
                _ => match lis.binary_search(&n) {
                    Err(j) => lis[j] = n,
                    _ => {}
                },
            }
        }

        return lis.len() as i32;
    }
}
// @lc code=end

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
