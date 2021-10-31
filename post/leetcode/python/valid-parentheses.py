# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/valid-parentheses
@Language: Python
@Datetime: 15-08-03 06:48
'''

class Solution:
    # @param {string} s A string
    # @return {boolean} whether the string is a valid parentheses
    def isValidParentheses(self, s):
        # Write your code here
        PL='({['
        PR=')}]'
        
        stack=[]
        for c in s:
            id=PL.find(c)
            if id !=-1:
                stack.append(id)
            elif len(stack)<1:
                return False
            else:
                if  stack[-1]==PR.find(c):
                    stack.pop()
                else:
                    return False
        
        if len(stack)>0:
            return False
            
        return True
                
            
