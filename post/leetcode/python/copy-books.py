# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/copy-books
@Language: Python
@Datetime: 15-07-20 13:28
'''

class Solution:
    # @param pages: a list of integers
    # @param k: an integer
    # @return: an integer
    def copyBooks(self, pages, k):
        # write your code here
        if k<2 :
            return sum(pages)
            
        if k>=len(pages):
            return max(pages)
            
        l=max(pages)
        r=sum(pages)
        
        while l<r:
            mid=(l+r)/2
            if self.getCutBooks(pages,mid)<=k:
                r=mid
            else:
                l=mid+1
        
        return r
        
    def getCutBooks(self, pages, x):
        count=1
        t=0
        for p in pages:
           t+=p;
           if t>x:
               count+=1
               t=p
        return count       
        
        
        
        
            
        
            
