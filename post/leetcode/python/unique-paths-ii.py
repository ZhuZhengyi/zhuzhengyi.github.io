# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/unique-paths-ii
@Language: Python
@Datetime: 15-07-28 14:21
'''

class Solution:
    """
    @param obstacleGrid: An list of lists of integers
    @return: An integer
    """
    def uniquePathsWithObstacles(self, obstacleGrid):
        # write your code here
        m=len(obstacleGrid)
        n=len(obstacleGrid[0])
            
        a=[([0]*n) for i in range(m)]
        for i in range(0,m):
            for j in range(0, n):
                if obstacleGrid[i][j]==1:
                    a[i][j]=0
                else:
                    if i==0 and j==0:
                        a[i][j]=1
                    elif i==0 and j!=0 :
                        a[i][j]=a[i][j-1]
                    elif j==0 and i!=0 :
                        a[i][j]=a[i-1][j]
                    elif i!=0 and j!=0:
                        a[i][j]=a[i-1][j]+a[i][j-1]
            
        return a[m-1][n-1]

