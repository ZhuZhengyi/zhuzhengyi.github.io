# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/majority-number
@Language: Python
@Datetime: 15-07-27 13:58
'''

class Solution:
    """
    @param nums: A list of integers
    @return: The majority number
    """
    def majorityNumber(self, nums):
        # write your code here
        
        while len(nums)>2:
            i=1
            while i<len(nums) :
                if nums[i]!=nums[0]:
                    nums.pop(i)
                    nums.pop(0)
                    break
                i+=1
            if i==len(nums):
                break
            
        return nums[0]
            
        
