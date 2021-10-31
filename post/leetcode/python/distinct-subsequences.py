# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/distinct-subsequences
@Language: Python
@Datetime: 15-08-29 11:21
'''

class Solution: 
    # @param S, T: Two string.
    # @return: Count the number of distinct subsequences
    def numDistinct(self, S, T):
        # write your code here
        # dp[i][j]: T[0-i] find in S[0-j]
        #
        #           / dp[i-1][j]     S[i]!=T[j]
        # dp[i][j]=|                 S[i]==T[j]
        #           \ dp[i-1][j]+dp[i-1][j-1] 
        #        |
        #        V
        #           / dp[j]          s[i]!=T[j]
        #  dp[j] = | 
        #           \ dp[j]+dp[j-1]  s[i]==T[j]

        m=len(S)
        n=len(T)
        
        if m==0 or n==0:
            return 0
            
        dp=[0]*(n+1)
        dp[0]=1
        for i in range(0, m):
            for j in range(n-1, -1, -1):
                if S[i]==T[j]:
                    dp[j+1]+=dp[j]
                
        return dp[n]