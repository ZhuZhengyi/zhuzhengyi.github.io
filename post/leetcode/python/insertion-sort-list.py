# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/insertion-sort-list
@Language: Python
@Datetime: 15-07-27 01:52
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
    @return: The head of linked list.
    """ 
    def insertionSortList(self, head):
        # write your code here
        h1=ListNode(0)
        t1=head
        h1.next=t1
        
        p=head.next
        t1.next=None
        
        while p!=None:
            t=p
            p=p.next
            t.next=None
            
            if t.val>=t1.val:
                t1.next=t
                t1=t
                continue
            
            i=h1
            while i!=None:
                if i.next==None:
                    i.next=t
                    t1=t
                    break
                elif i.next.val>t.val:
                    t.next=i.next
                    i.next=t
                    break
                i=i.next
                
        return h1.next
                