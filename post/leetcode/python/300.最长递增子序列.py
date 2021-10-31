#
# @lc app=leetcode.cn id=300 lang=python3
#
# [300] 最长递增子序列
#
# https://leetcode-cn.com/problems/longest-increasing-subsequence/description/
#
# algorithms
# Medium (51.75%)
# Likes:    1963
# Dislikes: 0
# Total Accepted:    371.6K
# Total Submissions: 717.9K
# Testcase Example:  '[10,9,2,5,3,7,101,18]'
#
# 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
# 
# 子序列是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7]
# 的子序列。
# 
# 
# 示例 1：
# 
# 
# 输入：nums = [10,9,2,5,3,7,101,18]
# 输出：4
# 解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
# 
# 
# 示例 2：
# 
# 
# 输入：nums = [0,1,0,3,2,3]
# 输出：4
# 
# 
# 示例 3：
# 
# 
# 输入：nums = [7,7,7,7,7,7,7]
# 输出：1
# 
# 
# 
# 
# 提示：
# 
# 
# 1 
# -10^4 
# 
# 
# 
# 
# 进阶：
# 
# 
# 你可以设计时间复杂度为 O(n^2) 的解决方案吗？
# 你能将算法的时间复杂度降低到 O(n log(n)) 吗?
# 
# 
#

# @lc code=start
class Solution:
    '''
    解法一：
    '''
    def lengthOfLIS(self, nums: List[int]) -> int:
        if len(nums) == 0: 
            return 0
        lis = [nums[0]]
        for n in nums[1:]:
            if n > lis[-1]:
                lis.append(n)
            else:
                l, r = 0, len(lis)
                while l < r:
                    i = l + (r - l) // 2
                    if lis[i] >= n and (i==0 or lis[i-1] < n ):
                        lis[i] = n
                        break
                    elif lis[i] < n:
                        l = i + 1
                    else:
                        r = i

        return len(lis)
# @lc code=end

