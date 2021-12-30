#
# @lc app=leetcode.cn id=31 lang=python3
#
# [31] 下一个排列
#
# https://leetcode-cn.com/problems/next-permutation/description/
#
# algorithms
# Medium (37.26%)
# Likes:    1455
# Dislikes: 0
# Total Accepted:    241.1K
# Total Submissions: 647.2K
# Testcase Example:  '[1,2,3]'
#
# 实现获取 下一个排列 的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列（即，组合出下一个更大的整数）。
# 
# 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
# 
# 必须 原地 修改，只允许使用额外常数空间。
# 
# 
# 
# 示例 1：
# 
# 
# 输入：nums = [1,2,3]
# 输出：[1,3,2]
# 
# 
# 示例 2：
# 
# 
# 输入：nums = [3,2,1]
# 输出：[1,2,3]
# 
# 
# 示例 3：
# 
# 
# 输入：nums = [1,1,5]
# 输出：[1,5,1]
# 
# 
# 示例 4：
# 
# 
# 输入：nums = [1]
# 输出：[1]
# 
# 
# 
# 
# 提示：
# 
# 
# 1 <= nums.length <= 100
# 0 <= nums[i] <= 100
# 
# 
#

from typing import List
# @lc code=start
class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        ## 解题思路
        * 从右至左查找到第一个nums[i], 使得nums[i-1]<nums[i]
        * 在i..len(nums) 中查找最小的nums[j], 并nums[j] > nums[i-1]
        * swap(nums[i-1], nums[j])；
        * sort(nums[i:])
        * 边界：
        """
        i = len(nums) - 1
        while i>0:
            if nums[i-1] < nums[i]:
                j = i
                bigger = j
                while j<len(nums): 
                    if nums[j] > nums[i-1] and nums[j] < nums[bigger]:
                        bigger = j 
                    j+=1
                nums[i-1], nums[bigger] = nums[bigger], nums[i-1]
                break
            i-=1
        if i == 0:
            nums.sort()
        else:
            nums[i:] = sorted(nums[i:])
    
if __name__ == "__main__":
    s = Solution()
    nums=[2,3,1]
    s.nextPermutation(nums)
    print(nums)

# @lc code=end

