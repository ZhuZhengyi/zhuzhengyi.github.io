# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/number-of-islands
@Language: Python
@Datetime: 15-07-31 12:04
'''

class Solution:
    # @param {boolean[][]} grid a boolean 2D matrix
    # @return {int} an integer
    def numIslands(self, grid):
        # Write your code here
        n=len(grid)
        if n<1:
            return 0
            
        m=len(grid[0])
        q=[]
        count=0
        
        for i in range(n):
            for j in range(m):
                if grid[i][j]==1:
                    q.append([i,j])
                    count+=1
                    while len(q)>0:
                        x,y=q[0]
                        q.pop(0)
                        grid[x][y]=-1
                        if y>0 and grid[x][y-1]==1:
                            q.append([x,y-1])
                        if x<n-1 and grid[x+1][y]==1:
                            q.append([x+1,y])
                        if y<m-1 and grid[x][y+1]==1:
                            q.append([x,y+1])
                        if x>0 and grid[x-1][y]==1:
                            q.append([x-1,y])
        return  count                    
