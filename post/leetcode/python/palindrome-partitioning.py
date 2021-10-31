# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/palindrome-partitioning
@Language: Python
@Datetime: 15-12-21 09:39
'''

class Solution:
    def isPalindrome(self,s,i,j):
        while i<=j and s[i]==s[j]:
                i+=1
                j-=1
        return i>j
    
    def dfs(self,s,res,path,start):
        n = len(s)
        if start==n:
            res.append(list(path))
        else:
            for i in range(start,n):
                if self.isPalindrome(s,start,i):
                    path.append(s[start:i+1])
                    self.dfs(s,res,path,i+1)
                    path.pop()
                    
    # @param s, a string
    # @return a list of lists of string
    def partition(self, s):
        # write your code here
        res=[]
        path=[]
        self.dfs(s,res,path,0)
            
        return res
    
    
   
        