# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/delete-digits
@Language: Python
@Datetime: 15-08-27 13:27
'''

class Solution:
    """
    @param A: A positive integer which has N digits, A is a string.
    @param k: Remove k digits.
    @return: A string
    """
    def DeleteDigits(self, A, k):
        # write you code here
        n=len(A)
        if n==1 and k==1:
            return ""
            
        if k==0:
            return A
        
        A=list(A)
        while k>0:
            i=0
            while i<len(A)-1 and A[i]<=A[i+1]:
                i+=1
            A.pop(i)
            k-=1
        
        while len(A)>0 and A[0]=='0':
            A.pop(0)
            
        return "".join(A)
        
