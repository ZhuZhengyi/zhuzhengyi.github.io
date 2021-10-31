# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/rotate-string
@Language: Python
@Datetime: 15-08-03 06:35
'''

class Solution:
    """
    param A: A string
    param offset: Rotate string with offset.
    return: Rotated string.
    """
    def rotateString(self, A, offset):
        # write you code here
        A=list(A)
        n=len(A)
        
        while n<1:
            return ''
        
        offset=offset%n
        
        #k=gcd(n,offset)
        gcd=lambda a,b: a if b==0 else gcd(b,a%b)
        k=gcd(n,offset)
        
        for i in range(k):
            s=i
            tmp=A[i]
            for _ in range(n/k):
                l=(s+offset)%n
                A[l],tmp=tmp,A[l]
                s=l
        
        return ''.join(A)
                
                
