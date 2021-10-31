#
# @lc app=leetcode.cn id=15 lang=python3
#
# [15] 三数之和
#
# https://leetcode-cn.com/problems/3sum/description/
#
# algorithms
# Medium (25.59%)
# Likes:    1827
# Dislikes: 0
# Total Accepted:    159.8K
# Total Submissions: 624.2K
# Testcase Example:  '[-1,0,1,2,-1,-4]'
#
# 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0
# ？找出所有满足条件且不重复的三元组。
# 
# 注意：答案中不可以包含重复的三元组。
# 
# 
# 
# 示例：
# 
# 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
# 
# 满足要求的三元组集合为：
# [
# ⁠ [-1, 0, 1],
# ⁠ [-1, -1, 2]
# ]
# 
# 
#

# @lc code=start
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n=len(nums)
        if n < 3:
            return []
        
        nums.sort()
        res=[]
        
        for i in range(n-2):
            if i>0 and nums[i]==nums[i-1]:
                continue
            j,k=i+1,n-1
            while j<k:
                if j>i+1 and nums[j]==nums[j-1]:
                    j+=1
                    continue
                if k<n-1 and nums[k]==nums[k+1]:
                    k-=1
                    continue
                
                s=nums[i]+nums[j]+nums[k]
            
                if s==0:
                    res.append([nums[i],nums[j],nums[k]])
                    j+=1
                    k-=1
                elif s<0:
                    j+=1
                else:
                    k-=1
        
        return res
        
# @lc code=end

