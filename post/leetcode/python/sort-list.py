# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/sort-list
@Language: Python
@Datetime: 15-08-18 14:29
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
    @return: You should return the head of the sorted linked list,
                  using constant space complexity.
    """
    def sortList(self, head):
        # write your code here
        if head == None:
            return None
        if head.next==None:
            return head
        
        dummy=ListNode(0)
        dummy.next=head
        
        slow=dummy
        fast=dummy.next
        while fast!=None and fast.next!=None:
            slow=slow.next
            fast=fast.next.next
        
        h2=slow.next
        slow.next=None
        
        h1=self.sortList(head)
        h2=self.sortList(h2)
        h=mergeLists(h1,h2)    
        
        return h
            
def mergeLists(h1,h2):
    dummy=ListNode(0)
    
    t=dummy
    while h1!=None and h2!=None:
        if h1.val < h2.val:
            t.next=h1
            h1=h1.next
        else:
            t.next=h2
            h2=h2.next
        t=t.next
    
    t.next=h1 if h1!=None else h2
    
    return dummy.next
                    
            
        
