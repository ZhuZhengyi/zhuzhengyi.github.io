# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/wiggle-sort
@Language: Python
@Datetime: 17-02-05 07:13
'''

class Solution(object):
    """
    @param {int[]} nums a list of integer
    @return nothing, modify nums in-place instead
    """
    def wiggleSort(self, nums):
        # Write your code here
        if len(nums) <= 1:
            return nums
        
        i = 1
        up = True
        while i < len(nums):
            if (nums[i] < nums[i-1]) == up:
                nums[i], nums[i-1] = nums[i-1], nums[i]
            up = not up
            i += 1