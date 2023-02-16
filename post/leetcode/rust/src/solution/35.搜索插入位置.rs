/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 *
 * https://leetcode-cn.com/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (45.97%)
 * Likes:    1229
 * Dislikes: 0
 * Total Accepted:    591.2K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
 * 
 * 请必须使用时间复杂度为 O(log n) 的算法。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 5
 * 输出: 2
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 2
 * 输出: 1
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 7
 * 输出: 4
 * 
 * 
 * 示例 4:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 0
 * 输出: 0
 * 
 * 
 * 示例 5:
 * 
 * 
 * 输入: nums = [1], target = 0
 * 输出: 0
 * 
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 
 * -10^4 
 * nums 为无重复元素的升序排列数组
 * -10^4 
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 二分查找
    /// * 注意边界条件
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() -1);
        // 边界条件
        if target > nums[r] {
            return (r + 1) as i32;
        }
        // 二分查找
        while l < r {
            let m = l + ( r - l) / 2 ;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }

        return l as i32;
    }
}
// @lc code=end

