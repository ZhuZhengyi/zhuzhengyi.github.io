# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-a-2d-matrix-ii
@Language: Python
@Datetime: 15-08-25 12:21
'''

class Solution:
    """
    @param matrix: An list of lists of integers
    @param target: An integer you want to search in matrix
    @return: An integer indicates the total occurrence of target in the given matrix
    """
    def searchMatrix(self, matrix, target):
        # write your code here
        m=len(matrix)
        if m==0:
            return 0
        n=len(matrix[0])
        
        count=0
        
        i,j=0,n-1
        while i<m and j>=0:
            if  target<matrix[i][j]:
                j-=1
            elif target>matrix[i][j]:
                i+=1
            elif target==matrix[i][j]:
                count+=1
                i+=1
                j-=1
        
        return count
            
        
            
                
                
                

