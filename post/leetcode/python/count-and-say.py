# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/count-and-say
@Language: Python
@Datetime: 15-08-01 12:15
'''

class Solution:
    def getnext(self, s):
        n=len(s)
        i=0
        ss=''
        while i<n:
            count=1
            j=i
            while j<n-1 and s[j+1]==s[j]:
                count+=1
                j+=1
            ss='%s%d%s'%(ss,count,s[i])
            i=j+1
        return ss
            
    # @param {int} n the nth
    # @return {string} the nth sequence
    def countAndSay(self, n):
        # Write your code here
        s='1'
        while n>1:
            s=self.getnext(s)
            n-=1
            
        return s
