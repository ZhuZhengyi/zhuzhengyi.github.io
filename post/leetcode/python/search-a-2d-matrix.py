# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-a-2d-matrix
@Language: Python
@Datetime: 15-08-03 08:18
'''

class Solution:
    """
    @param matrix, a list of lists of integers
    @param target, an integer
    @return a boolean, indicate whether matrix contains target
    """
    def searchMatrix(self, matrix, target):
        # write your code here
        m=len(matrix)
        if m<1:
            return False
        
        n=len(matrix[0])
        
        l,r=0,m*n-1
        
        while l<=r:
            mid=(l+r)/2
            x,y=mid/n,mid%n
            if target == matrix[x][y]:
                return True
            elif target < matrix[x][y]:
                r=mid-1
            else:
                l=mid+1
        
        return False

