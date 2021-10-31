# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/hash-function
@Language: Python
@Datetime: 15-07-27 00:57
'''

class Solution:
    """
    @param key: A String you should hash
    @param HASH_SIZE: An integer
    @return an integer
    """
    def hashCode(self, key, HASH_SIZE):
        # write your code here
        
        val=0
        for c in key:
            val=(val*33+ord(c))%HASH_SIZE
            
        return val%HASH_SIZE