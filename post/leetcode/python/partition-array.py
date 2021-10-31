# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/partition-array
@Language: Python
@Datetime: 15-08-09 09:13
'''

class Solution:
    """
    @param nums: The integer array you should partition
    @param k: As description
    @return: The index after partition
    """
    def partitionArray(self, nums, k):
        # write your code here
        # you should partition the nums by k
        # and return the partition index as description
        n=len(nums)
        if n==0:
            return 0
            
        c1,c2=0,0
        
        l,r=0,n-1
        while l<r:
            while r>l and nums[r]>=k:
                r-=1
            while l<r and nums[l]<k:
                l+=1
            if l<r:
                nums[l],nums[r]=nums[r],nums[l]
                l+=1
                r-=1
        
        if nums[l]<k:
            return r+1
        else:
            return l
        
            
        
        

