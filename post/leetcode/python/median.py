# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/median
@Language: Python
@Datetime: 15-07-29 11:25
'''

class Solution:
    """
    @param nums: A list of integers.
    @return: An integer denotes the middle number of the array.
    """
    def median(self, nums):
        # write your code here
        n=len(nums)
        k=(n-1)/2
        l,r=0,n-1
        while 1:
            flag=nums[l]
            l1,r1=l,r
            while l<r:
                while r>l and nums[r]>=flag:
                    r-=1
                if r>l:
                    nums[l]=nums[r]
                    l+=1
                while l<r and nums[l]<flag:
                    l+=1
                if l<r:
                    nums[r]=nums[l]
                    r-=1
            nums[l]=flag
            if l==k:
                return nums[l]
            elif l>k:
                l,r=l1,r-1
            else:
                l,r=l+1,r1
            
        
            
        
            
            