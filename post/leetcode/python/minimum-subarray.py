# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/minimum-subarray
@Language: Python
@Datetime: 15-08-02 23:29
'''

class Solution:
    """
    @param nums: a list of integers
    @return: A integer denote the sum of minimum subarray
    """
    def minSubArray(self, nums):
        # write your code 
        n=len(nums)
        if n<1:
            return None
        if n<2:
            return nums[0];
        
        sum_min=nums[0]
        cur_min=nums[0]
        for d in nums[1:]:
            sum_min=min(sum_min+d, d)
            cur_min=min(sum_min, cur_min)
            
        return cur_min

