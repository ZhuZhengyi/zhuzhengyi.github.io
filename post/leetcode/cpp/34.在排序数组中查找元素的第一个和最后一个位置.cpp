/*
 * @lc app=leetcode.cn id=34 lang=cpp
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 *
 * https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
 *
 * algorithms
 * Medium (39.03%)
 * Likes:    343
 * Dislikes: 0
 * Total Accepted:    71.2K
 * Total Submissions: 181.7K
 * Testcase Example:  '[5,7,7,8,8,10]\n8'
 *
 * 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
 * 
 * 你的算法时间复杂度必须是 O(log n) 级别。
 * 
 * 如果数组中不存在目标值，返回 [-1, -1]。
 * 
 * 示例 1:
 * 
 * 输入: nums = [5,7,7,8,8,10], target = 8
 * 输出: [3,4]
 * 
 * 示例 2:
 * 
 * 输入: nums = [5,7,7,8,8,10], target = 6
 * 输出: [-1,-1]
 * 
 */

// @lc code=start
class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int i = lower_bound(nums, target);
        int j = lower_bound(nums, target+1)-1;
        if (i< nums.size() && nums[i] == target) {
            return {i, j};
        }
        
        return {-1, -1};
    }

    int lower_bound(vector<int> &nums, int target) {
        int l = 0, r = nums.size() - 1; 
        while(l <= r) {
            int mid = l + (r-l)/2 ;
            if (nums[mid] < target) {
                l = mid + 1;
            } else {
                r = mid -1 ;
            }
        }
        return l;
    }
};
// @lc code=end

