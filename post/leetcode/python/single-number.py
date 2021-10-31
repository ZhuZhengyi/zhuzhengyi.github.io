# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/single-number
@Language: Python
@Datetime: 15-07-28 12:39
'''

class Solution:
    """
    @param A : an integer array
    @return : a integer
    """
    def singleNumber(self, A):
        # write your code here
        if len(A)<1:
            return 0
        while len(A)>1:
            found=False
            i=1
            while i<len(A) and not found :
                if A[i]==A[0]:
                    A.pop(i)
                    A.pop(0)
                    found=True
                i+=1
            if not found:
                break
            #if i==len(A):
            #    break
            
        return A[0]
