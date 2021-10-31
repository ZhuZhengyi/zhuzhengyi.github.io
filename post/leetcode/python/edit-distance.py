# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/edit-distance
@Language: Python
@Datetime: 15-08-22 12:22
'''

class Solution: 
    # @param word1 & word2: Two string.
    # @return: The minimum number of steps.
    def minDistance(self, word1, word2):
        # write your code here
        n1,n2=len(word1),len(word2)
        
        if n1==0:
            return n2
        
        if n2==0:
            return n1
            
        r=[[0]*(n2+1) for _ in range(n1+1)]
        
        for i in range(n1+1):
            r[i][0]=i
        for j in range(n2+1):
            r[0][j]=j
        
        for i in range(1, n1+1):
            for j in range(1, n2+1):
                if word1[i-1] == word2[j-1]:
                    r[i][j]=r[i-1][j-1]
                else:
                    r[i][j]=min(r[i-1][j],r[i][j-1],r[i-1][j-1])+1
                    
        return r[n1][n2]
