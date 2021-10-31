# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/partition-array-by-odd-and-even
@Language: Python
@Datetime: 15-07-30 22:29
'''

class Solution:
    # @param nums: a list of integers
    # @return: nothing
    def partitionArray(self, nums):
        # write your code here
        n=len(nums)
        
        if n<2:
            return
        
        i,j=0,n-1
        while i<j:
            while i<j and nums[i]%2==1:
                i+=1
            while j>i and nums[j]%2==0:
                j-=1
            if i<j:
                nums[i],nums[j]=nums[j],nums[i]
        
        

