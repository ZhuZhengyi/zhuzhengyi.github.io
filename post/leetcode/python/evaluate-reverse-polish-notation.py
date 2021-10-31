# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/evaluate-reverse-polish-notation
@Language: Python
@Datetime: 15-08-18 15:37
'''

class Solution:
    # @param {string[]} tokens The Reverse Polish Notation
    # @return {int} the value
    def evalRPN(self, tokens):
        # Write your code here
        import operator
        op={
            "+":lambda y,x:x+y,
            "-":lambda y,x:x-y,
            "*":lambda y,x:x*y,
            "/":lambda y,x:int(operator.truediv(x,y)),
            }
        s=[]
        for t in tokens:
            if t in op:
                s.append(op[t](s.pop(), s.pop()))
            else:
               s.append(int(t))
             
        return s.pop()
