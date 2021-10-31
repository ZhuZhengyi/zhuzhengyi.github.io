# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/maximal-square
@Language: Python
@Datetime: 15-08-07 12:40
'''

class Solution:
    #param matrix: a matrix of 0 and 1
    #return: an integer
    def maxSquare(self, matrix):
        # write your code here
        n=len(matrix)       #row
        if n==0:
            return 0
        
        m=len(matrix[0])    #col
        d=[[0]*(m+1) for _ in range(n+1)]
        
        max_sq=0
        for i in range(n):
            for j in range(m):
                if matrix[i][j]==1:
                    d[i+1][j+1]=min(d[i+1][j],d[i][j+1], d[i][j])+1
                    max_sq=max(max_sq, d[i+1][j+1]*d[i+1][j+1])
                
                    
        return max_sq
                    
        
        
        
