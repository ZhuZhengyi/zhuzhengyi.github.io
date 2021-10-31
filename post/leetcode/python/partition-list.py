# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/partition-list
@Language: Python
@Datetime: 15-07-27 01:18
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
    @param x: an integer
    @return: a ListNode 
    """
    def partition(self, head, x):
        # write your code here
        h1=t1=ListNode(0)
        h2=t2=ListNode(0)
    
        if head==None:
            return None
        
        if head.next == None:
            return head
    
        p=head
        while p!=None:
            t=p
            p=p.next
            if t.val<x:
                t1.next=t
                t1=t1.next
            else:
                t2.next=t
                t2=t2.next
        
        t1.next=h2.next
        t2.next=None
        
        return h1.next
        
                
