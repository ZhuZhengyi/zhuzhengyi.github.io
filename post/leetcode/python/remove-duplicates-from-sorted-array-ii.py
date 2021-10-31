# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-duplicates-from-sorted-array-ii
@Language: Python
@Datetime: 15-07-27 13:01
'''

class Solution:
    """
    @param A: a list of integers
    @return an integer
    """
    def removeDuplicates(self, A):
        # write your code here
        n=len(A)
        if n<3:
            return n

        r=n
        t=1
        for i in range(2,n):
            if A[i]!=A[t]:
                A[t+1]=A[i]
                t+=1
            elif A[i]==A[t] and A[i]!=A[t-1]:
                A[t+1]=A[i]
                t+=1
            else:
                r-=1
        return r



