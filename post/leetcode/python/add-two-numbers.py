# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/add-two-numbers
@Language: Python
@Datetime: 15-07-26 08:06
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    # @param l1: the first list
    # @param l2: the second list
    # @return: the sum list of l1 and l2 
    def addLists(self, l1, l2):
        # write your code here
        
        c=0
        r=t=ListNode(-1)
        while l1!=None or l2!=None:
            if l1!=None:
                a=l1.val; l1=l1.next 
            else:
                a=0
            if l2!=None:
                b=l2.val; l2=l2.next
            else:
                b=0
            s=(a+b+c)
            c=1 if s>9 else 0
            
            t.next=ListNode(s%10)
            t=t.next
        if c>0:
            t.next=ListNode(c)
            t=t.next
        
        return r.next
        
        
            