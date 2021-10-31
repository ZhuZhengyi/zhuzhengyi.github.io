# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/first-position-of-target
@Language: Python
@Datetime: 15-08-03 08:19
'''

class Solution:
    # @param nums: The integer array
    # @param target: Target number to find
    # @return the first position of target in nums, position start from 0 
    def binarySearch(self, nums, target):
        # write your code here
        l,r=0,len(nums)-1
        res=-1
        while l<=r:
            mid=(l+r)/2
            if target==nums[mid]:
                res=mid
                break
            elif target<nums[mid]:
                r=mid-1
            else:
                l=mid+1
        
        if res>0:
            for i in range(res-1,-1,-1):
                if nums[i] != target:
                    return i+1
                
        return res
