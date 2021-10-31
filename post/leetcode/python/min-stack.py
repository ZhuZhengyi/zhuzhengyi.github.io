# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/min-stack
@Language: Python
@Datetime: 15-07-26 00:46
'''

class MinStack(object):

    def __init__(self):
        # do some intialize if necessary
        self.s=[]
        self.sm=[]

    def push(self, number):
        # write yout code here
        self.s.append(number)
        if len(self.sm) < 1:
            self.sm.append(number)
        else:
            self.sm.append(min(self.sm[-1], number))

    def pop(self):
        # pop and return the top item in stack
        t=self.s[-1]
        self.s.pop()
        self.sm.pop()
        return t

    def min(self):
        # return the minimum number in stack
        return self.sm[-1]