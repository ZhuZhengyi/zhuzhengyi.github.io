# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/unique-binary-search-trees
@Language: Python
@Datetime: 15-08-09 08:17
'''

class Solution:
    # @paramn n: An integer
    # @return: An integer
    def numTrees(self, n):
        # write your code here
        if n==0:
            return 1
        if n==1:
            return 1
            
        f=[0]*(n+1)
        f[:2]=[1,1]
        for i in range(2,n+1):
            for j in range(0, i):
                f[i]+=(f[j]*f[i-j-1])
            
        return f[n]