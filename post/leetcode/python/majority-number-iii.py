# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/majority-number-iii
@Language: Python
@Datetime: 15-08-11 14:02
'''

class Solution:
    """
    @param nums: A list of integers
    @param k: As described
    @return: The majority number
    """
    def majorityNumber(self, nums, k):
        # write your code here
        
        h={}
        for i in nums:
            if h.get(i)==None:
                h[i]=1
            else:
                h[i]+=1
            
            if len(h)==k:
                for i,v in h.items():
                    h[i]-=1
                    if h[i]==0:
                        h.pop(i)
                        
        for i,v in h.items():
            h[i]=0
        
        for i in nums:
            if h.get(i)!=None:
                h[i]+=1
                
        for i,v in h.items():
            if v>len(nums)/k:
                return i 
    
        return 0
            
