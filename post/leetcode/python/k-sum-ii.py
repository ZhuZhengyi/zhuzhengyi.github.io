# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/k-sum-ii
@Language: Python
@Datetime: 15-08-05 21:55
'''

class Solution:
    """
    @param A: An integer array.
    @param k: A positive integer (k <= length(A))
    @param target: Integer
    @return a list of lists of integer 
    """
    def kSumII(self, A, k, target):
        # write your code here
        cur=[]
        res=[]
        
        getKSum(A, k, target, cur, res, 0, 0)
                
        return res
    

def getKSum(A, k, target, cur, res, i, sum):
    if k==len(cur) and sum==target:
        res.append(list(cur))
        return
    
    if i==len(A):
        return
    
    if sum>target:
        return
    
    cur.append(A[i])
    getKSum(A, k, target, cur, res, i+1, sum+A[i])
    cur.pop()
    getKSum(A, k, target, cur, res, i+1, sum)
    