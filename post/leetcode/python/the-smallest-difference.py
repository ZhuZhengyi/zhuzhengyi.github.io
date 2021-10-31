# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/the-smallest-difference
@Language: Python
@Datetime: 15-08-16 05:34
'''

class Solution:
    # @param A, B: Two lists of integer
    # @return: An integer
    def smallestDifference(self, A, B):
        # write your code here
        A.sort()
        B.sort()
        
        res=abs(A[0]-B[0])
        
        for a in A:
            l,r=0,len(B)-1
            while l+1<r:
                m=(l+r)/2
                if a==B[m]:
                    return 0
                elif a<B[m]:
                    r=m
                else:
                    l=m
            #if l<r:
            res=min(res, abs(a-B[l]), abs(B[r]-a))
            
        return res
