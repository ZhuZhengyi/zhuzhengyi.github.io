# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/reorder-list
@Language: Python
@Datetime: 15-08-18 14:45
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
    @return: nothing
    """
    def reorderList(self, head):
        # write your code here
        if head==None:
            return None
        
        if head.next==None:
            return head
        
        # get mid ptr
        mid=head
        tail=mid.next
        while tail!=None and tail.next!=None:
            mid=mid.next
            tail=tail.next.next
            
        # reverse tail half
        t=mid.next
        mid.next=None
        
        h2=None
        while t!=None:
            p=t.next
            t.next=h2
            h2=t
            t=p
        
        # merge h1, h2
        h1=head
        while h1!=None and h2 !=None:
            tmp=h2.next
            h2.next=h1.next
            h1.next=h2
            h1=h2.next
            h2=tmp
        
        return head
        
            
            

