# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/minimum-path-sum
@Language: Python
@Datetime: 15-08-03 00:22
'''

class Solution:
    """
    @param grid: a list of lists of integers.
    @return: An integer, minimizes the sum of all numbers along its path
    """
    def minPathSum(self, grid):
        # write your code here
        m=len(grid)     #m
        if m<1:
            return 0
        if m==1:
            return sum(grid[0])
        
        n=len(grid[0])  #n
        if n<1:
            return 0
        
        path=[[grid[0][0]]*n for _ in range(m) ]
        for i in range(m):      #i
            for j in range(n):  #j
                if i==0 and j==0:
                    path[i][j]=grid[i][j]
                elif i==0 and j>0:
                    path[i][j]=path[i][j-1]+grid[i][j]
                elif j==0 and i>0:
                    path[i][j]=path[i-1][j]+grid[i][j]
                else:
                    path[i][j]=min(path[i][j-1], path[i-1][j])+grid[i][j]
                    
        return path[m-1][n-1]
