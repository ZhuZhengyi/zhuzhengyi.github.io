# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/ugly-number-ii
@Language: Python
@Datetime: 15-08-09 15:01
'''

class Solution:
    """
    @param k: The number k.
    @return: The kth prime number as description.
    """
    def kthPrimeNumber(self, k):
        # write your code here
        if k==0:
            return 1
        
        PNS=[0]*(k+1)
        m3,m5,m7=0,0,0
        PNS[0]=1
        
        for i in range(1,k+1):
            min_p=min(PNS[m3]*3, PNS[m5]*5, PNS[m7]*7)
            PNS[i]=min_p
            
            while PNS[m3]*3 <= min_p:
                m3+=1
            while PNS[m5]*5 <= min_p:
                m5+=1
            while PNS[m7]*7 <= min_p:
                m7+=1
        
        return PNS[k]
            
