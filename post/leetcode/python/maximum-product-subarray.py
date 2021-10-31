# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/maximum-product-subarray
@Language: Python
@Datetime: 15-08-12 13:19
'''

class Solution:
    # @param nums: an integer[]
    # @return: an integer
    def maxProduct(self, nums):
        # write your code here
        
        n=len(nums)
        
        max_p,min_p=nums[0],nums[0]
        t=nums[0]
        
        for i in nums[1:]:
            #max_p1=max_p
            max_p,min_p=max(i, max_p*i, min_p*i),min(i, min_p*i, max_p*i)
            t=max(max_p, t)
            
        return t
