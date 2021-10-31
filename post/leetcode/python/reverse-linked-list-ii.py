# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/reverse-linked-list-ii
@Language: Python
@Datetime: 15-08-19 14:45
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
    @param head: The head of linked list
    @param m: start position
    @param n: end position
    """
    def reverseBetween(self, head, m, n):
        # write your code here
        
        dummy=ListNode(0)
        dummy.next=head
        
        t1=dummy
        p1=t1.next
        m1=m
        while m1>1:
            m1-=1
            t1=t1.next
            p1=p1.next
        
        p2=p1
        t=None
        s=n-m
        while s>=0:
            s-=1
            tmp=p2.next
            p2.next=t
            t=p2
            p2=tmp
            
        t1.next=t
        p1.next=p2
        
        return dummy.next
