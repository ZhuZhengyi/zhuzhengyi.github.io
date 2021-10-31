# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/matrix-zigzag-traversal
@Language: Python
@Datetime: 15-08-02 09:17
'''

class Solution:
    # @param: a matrix of integers
    # @return: a list of integers
    def printZMatrix(self, matrix):
        # write your code here
        m=len(matrix)
        if m<1:
            return None
        
        n=len(matrix[0])
        if m==1 and n>0:
            return matrix[0]
        
        res=[]
        i,j=0,0
        while 1:
            res.append(matrix[i][j])
            if i==m-1 and j==n-1:
                break
            if (i+j)%2==0:
                if i==0 or j==n-1:
                    if j!=n-1:
                        j+=1
                    else:
                        i+=1
                else:
                    i,j=i-1,j+1
            else:
                if i==m-1 or j==0:
                    if i<m-1:
                        i+=1
                    else:
                        j+=1
                else:
                    i,j=i+1,j-1
                
        return res
        
        
