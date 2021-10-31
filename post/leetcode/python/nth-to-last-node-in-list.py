# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/nth-to-last-node-in-list
@Language: Python
@Datetime: 15-07-28 12:50
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
    @return: Nth to last node of a singly linked list. 
    """
    def nthToLast(self, head, n):
        # write your code here
        p1=head
        p2=p1
        for i in range(n):
            p2=p2.next
            
        while p2!=None:
            p1=p1.next
            p2=p2.next
            
        return p1
