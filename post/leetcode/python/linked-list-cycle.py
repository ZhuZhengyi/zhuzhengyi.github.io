# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/linked-list-cycle
@Language: Python
@Datetime: 15-08-16 05:42
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
    @param head: The first node of the linked list.
    @return: True if it has a cycle, or false
    """
    def hasCycle(self, head):
        # write your code here
        if head==None:
            return False

        pFast=pSlow=head
        while pSlow.next!=None and pFast.next!=None and pFast.next.next!=None:
            pSlow=pSlow.next
            pFast=pFast.next.next
            if pFast==pSlow:
                return True

        return False

