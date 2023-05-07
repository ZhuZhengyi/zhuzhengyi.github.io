/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 *
 * https://leetcode.cn/problems/maximum-subarray/description/
 *
 * algorithms
 * Medium (54.86%)
 * Likes:    5842
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 2.4M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
 *
 * 子数组 是数组中的一个连续部分。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
 * 输出：6
 * 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1]
 * 输出：1
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [5,4,-1,7,8]
 * 输出：23
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 *
 *
 * 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 贪心法
    /// 1. 从左至右依次遍历nums;
    /// 2. 令cur_sub: 表示以当前元素时为末尾元素的最大连续子数组和;
    /// 3. 令max_sub: 表示到当前为止的最大连续子数组和;
    /// 4. cur_sub = max(cur_sub+n, n)
    /// 5  max_sub = max(max_sub, cur_sub)
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut cur_sub = 0; //当前临时子数组和
        let mut max_sub = i32::MIN; //最大子数组和
        for n in nums {
            cur_sub = std::cmp::max(cur_sub + n, n);
            max_sub = std::cmp::max(max_sub, cur_sub);
        }
        max_sub
    }
}
// @lc code=end

struct Solution;
