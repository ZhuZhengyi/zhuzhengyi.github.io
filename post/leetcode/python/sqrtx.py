# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/sqrtx
@Language: Python
@Datetime: 15-07-27 02:00
'''

class Solution:
    """
    @param x: An integer
    @return: The sqrt of x
    """
    def sqrt(self, x):
        # write your code here
        if x==0:
            return 0
        if x<4:
            return 1
        l,r=1,x/2
        res=(l+r)/2
        while l<r:
            res=(l+r)/2
            if res*res<=x and (res+1)*(res+1)>x:
                return res
            elif res*res>x:
                r=res
            else:
                l=res
                
        return res
            
        
        
