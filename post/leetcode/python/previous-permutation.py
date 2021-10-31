# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/previous-permutation
@Language: Python
@Datetime: 15-08-09 07:17
'''

class Solution:
    # @param num :  a list of integer
    # @return : a list of integer
    def previousPermuation(self, num):
        # write your code here
        n=len(num)
        
        if n<1:
            return None
        if n==1:
            return num
        if n==2:
            return num[::-1]
        
        # r->l, 1st ascend num
        i=n-1
        while i>0 and num[i-1]<=num[i]:
            i-=1
            
        if i==0:
            return num[::-1]
        
        k=n-1
        while k>i and num[k]>=num[i-1]:
            k-=1
        
        num[i-1],num[k]=num[k],num[i-1]
        b=num[i:]
        
        return num[:i]+b[::-1]
        
        
            
            

