# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/heapify
@Language: Python
@Datetime: 15-08-26 12:59
'''

class Solution:
    # @param A: Given an integer array
    # @return: void
    def heapify(self, A):
        # write your code here
        n=len(A)

        for p in range(n>>1, -1, -1):
            #
            while (p*2+1)<n:
                l=p*2+1
                r=l+1
                if r<n and A[l]>A[r]:
                    l=r
                if A[p]>A[l]:
                    A[p],A[l]=A[l],A[p]
                p=l
                
