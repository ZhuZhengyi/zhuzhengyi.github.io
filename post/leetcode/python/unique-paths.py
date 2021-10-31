# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/unique-paths
@Language: Python
@Datetime: 15-08-28 12:39
'''

class Solution:
    """
    @param n and m: positive integer(1 <= n , m <= 100)
    @return an integer
    """ 
    def uniquePaths(self, m, n):
        # write your code here
        if m==1 or n==1:
            return 1
            
        a=[([0]*n) for i in range(m)]
        b=[0]*n
        for i in range(m):
            for j in range(n):
                if i==0 or j==0:
                    b[j]=1     
                    a[i][j]=1
                else:
                    b[j]+=b[j-1]
                    a[i][j]=a[i-1][j]+a[i][j-1]
            
        return b[n-1]   
        #return a[m-1][n-1]