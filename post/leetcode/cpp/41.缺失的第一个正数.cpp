/*
 * @lc app=leetcode.cn id=41 lang=cpp
 *
 * [41] 缺失的第一个正数
 *
 * https://leetcode-cn.com/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (42.50%)
 * Likes:    1461
 * Dislikes: 0
 * Total Accepted:    219.1K
 * Total Submissions: 515K
 * Testcase Example:  '[1,2,0]'
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,0]
 * 输出：3
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * -2^31 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * 将nums排列为[1,2,..n]的形式，缺失的第一个正整数x，其下标为x-1;
    * 将x=nums[i]交换到nums[x](0<x<n)的位置;
    */
    int firstMissingPositive(vector<int>& nums) {
        int n = nums.size();
        for(int i=0; i<n; i++) {
            while(nums[i]>0 && nums[i]<=n && nums[nums[i]-1] != nums[i]) {
                swap(nums[nums[i]-1], nums[i]);
            }
        }

        for(int i=0; i<n; i++) {
            if (nums[i] != i+1) {
                return i+1;
            }
        }

        return n+1;
    }
};
// @lc code=end

