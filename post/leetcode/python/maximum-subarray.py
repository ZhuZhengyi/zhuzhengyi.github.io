# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/maximum-subarray
@Language: Python
@Datetime: 15-08-02 22:42
'''

class Solution:
    """
    @param nums: A list of integers
    @return: An integer denote the sum of maximum subarray
    """
    def maxSubArray(self, nums):
        # write your code here
        n=len(nums)
        
        s=nums[0]
        t=nums[0]
        for n in nums[1:]:
            s=max(s+n, n)
            t=max(s,t)
            
        return t
        
