# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-common-subsequence
@Language: Python
@Datetime: 15-08-16 02:41
'''

class Solution:
    """
    @param A, B: Two strings.
    @return: The length of longest common subsequence of A and B.
    """
    def longestCommonSubsequence(self, A, B):
        # write your code here
        la,lb=len(A),len(B)
        d=[[0]*(lb+1) for _ in range(la+1)]
        
        for i in range(la):
            for j in range(lb):
                if A[i]==B[j]:
                    d[i+1][j+1]=d[i][j]+1
                else:
                    d[i+1][j+1]=max(d[i+1][j],d[i][j+1])
        
        return d[la][lb]

