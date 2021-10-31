# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/interleaving-string
@Language: Python
@Datetime: 15-08-23 09:42
'''

class Solution:
    """
    @params s1, s2, s3: Three strings as description.
    @return: return True if s3 is formed by the interleaving of
             s1 and s2 or False if not.
    @hint: you can use [[True] * m for i in range (n)] to allocate a n*m matrix.
    """
    def isInterleave(self, s1, s2, s3):
        # write your code here
        n1,n2,n3=len(s1),len(s2),len(s3)
        
        if n1+n2!=n3:
            return False
            
        res=[ [False]*(n2+1) for _ in range(n1+1)]
        
        res[0][0]=True
        
        for i in range(n1):
            if s1[i]==s3[i]:
                res[i+1][0]=True
        for j in range(n2):
            if s2[j]==s3[j]:
                res[0][j+1]=True
        
        for i in range(1,n1+1):
            for j in range(1, n2+1):
                res[i][j]=(res[i-1][j] and s1[i-1]==s3[i+j-1] ) or ( res[i][j-1] and s2[j-1]==s3[i+j-1])
        
        return res[n1][n2]
