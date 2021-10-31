# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/next-permutation
@Language: Python
@Datetime: 15-08-09 05:09
'''

class Solution:
    # @param num :  a list of integer
    # @return : a list of integer
    def nextPermutation(self, num):
        # write your code here
        n=len(num)
        
        if n==0:
            return None
        if n==1:
            return num
        if n==2:
            num.reverse()
            return num
        
        i=n-1
        while i>0 and num[i-1]>=num[i]:
            i-=1
            
        if i==0:
            num.reverse()
            return num
            
        k=n-1
        while k>i-1 and num[k]<=num[i-1]:
            k-=1
        
        # swap i-1, k
        num[i-1],num[k]=num[k],num[i-1]
        
        # 
        b=num[i:]
        num=num[:i]+b[::-1]
        
        return num
        

