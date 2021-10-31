# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/find-the-missing-number
@Language: Python
@Datetime: 15-07-20 12:05
'''

class Solution:
    # @param nums: a list of integers
    # @return: an integer
    def findMissing(self, nums):
        # write your code here
        n=len(nums)
        return n*(n+1)/2-sum(nums)
