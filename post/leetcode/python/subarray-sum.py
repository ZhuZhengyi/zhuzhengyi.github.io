# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/subarray-sum
@Language: Python
@Datetime: 15-08-23 07:20
'''

class Solution:
    """
    @param nums: A list of integers
    @return: A list of integers includes the index of the first number 
             and the index of the last number
    """
    def subarraySum(self, nums):
        # write your code here
        n=len(nums)
        
        if n<1:
            return None
        
        for i in range(n):
            s=0
            for j in range(i, n):
                if s+nums[j]==0:
                    return [i,j]
                else:
                    s+=nums[j]
        
        return None
                    
