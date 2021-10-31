# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-duplicates-from-sorted-list
@Language: Python
@Datetime: 15-07-26 00:32
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
    @param head: A ListNode
    @return: A ListNode
    """
    def deleteDuplicates(self, head):
        # write your code here
        t=head
        if t==None:
            return None
        while t.next !=None:
            if t.next.val == t.val:
                t.next=t.next.next
            else:
                t=t.next
        return head
