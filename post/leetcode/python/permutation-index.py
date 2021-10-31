# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/permutation-index
@Language: Python
@Datetime: 15-08-05 22:49
'''

class Solution:
    # @param {int[]} A an integer array
    # @return {long} a long integer
    def permutationIndex(self, A):
        # Write your code here
        n=len(A)
        res=0
        
        if n<1:
            return 0
        if n==1:
            return 1
        
        count=0
        
        for a in A[1:]:
            if A[0]>a:
                count+=1
                
        #1*2*..*(n-1)
        p=lambda n: reduce(lambda x,y: x*y , range(1, n) )
        
        #
        return count*p(n)+self.permutationIndex(A[1:])
        
