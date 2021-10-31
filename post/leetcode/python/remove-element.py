# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-element
@Language: Python
@Datetime: 15-07-21 14:35
'''

class Solution:
    """
    @param A: A list of integers
    @param elem: An integer
    @return: The new length after remove
    """
    def removeElement(self, A, elem):
        # write your code here
        n=len(A)
        if n<1 :
            return 0
        
        count=0
        for i in range(n):
            if A[i]==elem:
                count+=1
            else:
                A[i-count]=A[i]

        A=A[:n-count]

        return n-count