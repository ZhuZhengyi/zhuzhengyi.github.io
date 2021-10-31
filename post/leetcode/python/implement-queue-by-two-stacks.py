# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/implement-queue-by-two-stacks
@Language: Python
@Datetime: 15-08-16 05:51
'''

class Queue:

    def __init__(self):
        self.stack1 = []
        self.stack2 = []
        
    def push(self, element):
        # write your code here
        self.stack1.append(element)
        
    def top(self):
        # write your code here
        # return the top element
        if len(self.stack2)==0:
            while len(self.stack1)>0:
                self.stack2.append(self.stack1.pop())
        
        if len(self.stack2)>0:
            return self.stack2[-1]
        else:
            return None

    def pop(self):
        # write your code here
        # pop and return the top element
        if len(self.stack2)==0:
            while len(self.stack1)>0:
                self.stack2.append(self.stack1.pop())
        
        if len(self.stack2)>0:
            return self.stack2.pop()
        else:
            return None
