# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/recover-rotated-sorted-array
@Language: Python
@Datetime: 15-08-01 13:38
'''

class Solution:
    """
    @param nums: The rotated sorted array
    @return: nothing
    """
    def recoverRotatedSortedArray(self, nums):
        # write your code here
        n=len(nums)
        
        if nums[-1]>nums[0]:
            return
        
        m=0
        while m<n-1 and nums[m+1]>=nums[m]:
            m+=1
        m+=1
        
        a,b=n,m
        while b>0:
            a,b=b,a%b
            
        for i in range(a):
            j=i
            count=0
            last=nums[i]
            while count<n/a:
                count+=1
                k=(j+n-m)%n
                last,nums[k]=nums[k],last
                j=k
            
            
            
            
            
        
                
            
            
            
                    
            
