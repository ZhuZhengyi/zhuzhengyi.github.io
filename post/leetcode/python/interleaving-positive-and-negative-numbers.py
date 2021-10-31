# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/interleaving-positive-and-negative-numbers
@Language: Python
@Datetime: 15-08-25 12:09
'''

class Solution:
    """
    @param A: An integer array.
    @return nothing
    """
    def rerange(self, A):
        # write your code here
        n=len(A)
        if n==0:
            return
        
        if n==1:
            return
        
        positive,negitive=0,0
        for a in A:
            if a>0:
                positive+=1
            else:
                negitive+=1
        
        neg,pos=0,1
        if 2*positive>n:
            pos,neg=0,1
        elif 2*negitive>n:
            neg,pos=0,1
        
        while pos<n and neg<n:
            while neg<n and A[neg]<0:
                neg+=2
            while pos<n and A[pos]>0:
                pos+=2
            if neg<n and pos<n and A[neg]>0 and A[pos]<0:
                A[pos],A[neg]=A[neg],A[pos]
      
