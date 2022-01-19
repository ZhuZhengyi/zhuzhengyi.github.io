#
# @lc app=leetcode.cn id=41 lang=python3
#
# [41] 缺失的第一个正数
#
# https://leetcode-cn.com/problems/first-missing-positive/description/
#
# algorithms
# Hard (42.06%)
# Likes:    1315
# Dislikes: 0
# Total Accepted:    185.4K
# Total Submissions: 440.9K
# Testcase Example:  '[1,2,0]'
#
# 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
# 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
# 
# 
# 
# 示例 1：
# 
# 
# 输入：nums = [1,2,0]
# 输出：3
# 
# 
# 示例 2：
# 
# 
# 输入：nums = [3,4,-1,1]
# 输出：2
# 
# 
# 示例 3：
# 
# 
# 输入：nums = [7,8,9,11,12]
# 输出：1
# 
# 
# 
# 
# 提示：
# 
# 
# 1 
# -2^31 
# 
# 
#

# @lc code=start
class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        '''
        ## 解题思路
        * 将数组每个元素n放置到下标为n-1的位置，缺失的第一个正数为下标不符合数下标i+1
        * 将每个数n移动到下标为n-1的地方;
         移动时，要注意：
         1. n要在数组范围内(0, len(nums)]；
         2. 已经是正确的不用移；
         3. 目标相等的不用移；
        * 边界条件
          1. [1, 1]
          2. [3, 4, -1, 1]
        '''
        for i in range(len(nums)):
            ## 将每个数n移动到下标为n-1的地方;
            while nums[i] > 0 and nums[i] <= len(nums) and nums[i] != i+1 and nums[i] != nums[nums[i]-1]:
                n = nums[i]
                nums[n-1], nums[i] = nums[i], nums[n-1]

        for i, n in enumerate(nums):
            if n != i+1:  #第一个值不为下标+1的
                return i+1

        ## 所有下标都符合，则缺失的数为数组长度+1
        return len(nums) + 1

# @lc code=end

