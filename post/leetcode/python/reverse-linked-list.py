# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/reverse-linked-list
@Language: Python
@Datetime: 15-07-26 07:51
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
    @return: You should return the head of the reversed linked list. 
                  Reverse it in-place.
    """
    def reverse(self, head):
        # write your code here
        h=None
        
        p=head
        while p!=None:
            t=p
            p=p.next
            
            t.next=h
            h=t
        
        return h
