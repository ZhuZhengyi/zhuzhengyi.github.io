# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/next-permutation-ii
@Language: Python
@Datetime: 15-08-09 07:32
'''

class Solution:
    # @param nums: a list of integer
    # @return: return nothing (void), do not return anything, modify nums in-place instead
    def nextPermutation(self, num):
        # write your code here
        n=len(num)

        if n==0:
            return
        if n==1:
            return
        if n==2:
            num.reverse()
            return

        # r->l 1st decrease num
        i=n-1
        while i>0 and num[i-1]>=num[i]:
            i-=1

        # none decrease num
        if i==0:
            num.reverse()
            return

        # r->l  1st > num[i-1]
        k=n-1
        while k>i-1 and num[k]<=num[i-1]:
            k-=1

        # swap i-1, k
        num[i-1],num[k]=num[k],num[i-1]

        # swap i:-1
        l, r=i,n-1
        while l<r:
            num[l],num[r]=num[r],num[l]
            l+=1
            r-=1
        
        
        
