# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/two-strings-are-anagrams
@Language: Python
@Datetime: 15-08-23 02:41
'''

class Solution:
    """
    @param s: The first string
    @param b: The second string
    @return true or false
    """
    def anagram(self, s, t):
        # write your code here
        
        if len(s)!=len(t):
            return False
        
        s=list(s)
        t=list(t)
        
        s.sort()
        t.sort()
                
        return s==t
