# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/move-zeroes
@Language: Python
@Datetime: 17-01-21 06:57
'''

class Solution:
    # @param {int[]} nums an integer array
    # @return nothing, do this in-place
    def moveZeroes(self, nums):
        # Write your code here
        zero_count = 0
        for i in range(len(nums)):
            if nums[i] == 0:
                zero_count += 1
            else:
                nums[i-zero_count] = nums[i]
        if zero_count > 0:
            for i in range(zero_count):
                nums[len(nums)-i-1] = 0
            