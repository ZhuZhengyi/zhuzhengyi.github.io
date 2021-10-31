#
# @lc app=leetcode.cn id=16 lang=python3
#
# [16] 最接近的三数之和
#
# https://leetcode-cn.com/problems/3sum-closest/description/
#
# algorithms
# Medium (43.03%)
# Likes:    367
# Dislikes: 0
# Total Accepted:    76.1K
# Total Submissions: 176.7K
# Testcase Example:  '[-1,2,1,-4]\n1'
#
# 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target
# 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
# 
# 例如，给定数组 nums = [-1，2，1，-4], 和 target = 1.
# 
# 与 target 最接近的三个数的和为 2. (-1 + 2 + 1 = 2).
# 
# 
#

# @lc code=start
class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        len = len(nums)
        len = 

        if len < 3:
            return 0

        nums = sort(nums)
        ans = sum(nums[:3])

        for i in range(0, len(nums)-2):
            l = i
            r = len
            res = target - nums[i]
            while l < r:
                pass
                
        pass
# @lc code=end

