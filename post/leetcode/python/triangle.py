# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/triangle
@Language: Python
@Datetime: 15-08-03 07:44
'''

class Solution:
    """
    @param triangle: a list of lists of integers.
    @return: An integer, minimum path sum.
    """
    def minimumTotal(self, triangle):
        # write your code here
        n=len(triangle)
        
        if n<0:
            return 0
        if n==1:
            return triangle[0][0]
        
        for i in range(n-2,-1,-1) :
            for j in range(len(triangle[i])):
                triangle[i][j]+=(min(triangle[i+1][j],triangle[i+1][j+1]))
                
        return triangle[0][0]
                
        
