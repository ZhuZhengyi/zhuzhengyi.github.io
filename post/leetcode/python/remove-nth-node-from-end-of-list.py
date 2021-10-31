# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-nth-node-from-end-of-list
@Language: Python
@Datetime: 15-07-27 14:28
'''

"""
Definition of ListNode
class ListNode(object):

    def __init__(self, val, next=None):
        self.val = val
        self.next = next
"""
class Solution:
    """
    @param head: The first node of linked list.
    @param n: An integer.
    @return: The head of linked list.
    """
    def removeNthFromEnd(self, head, n):
        # write your code here
        
        p=t=head
        for i in range(n):
            t=t.next
        
        while t!=None and t.next!=None:
            p=p.next
            t=t.next
          
        if p==head:
            return p.next
            
        p.next=p.next.next
        
        return head
