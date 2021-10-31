# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/strstr
@Language: Python
@Datetime: 15-08-23 07:35
'''

class Solution:
    def strStr(self, source, target):
        # write your code here
        
        if source==None or target==None:
            return -1
        
        n,m=len(source),len(target)
        
        if n<m:
            return -1
        
        for i in range(n-m+1):
            k=i
            j=0
            while k<n and j<m and source[k]==target[j]:
                k+=1
                j+=1
            if j==m:
                return i
            
        
        return -1
