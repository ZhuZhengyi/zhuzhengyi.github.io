# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/majority-number-ii
@Language: Python
@Datetime: 15-08-11 14:03
'''

class Solution:
    """
    @param nums: A list of integers
    @return: The majority number occurs more than 1/3
    """
    def majorityNumber(self, nums):
        # write your code here
        k=3
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
            
            

