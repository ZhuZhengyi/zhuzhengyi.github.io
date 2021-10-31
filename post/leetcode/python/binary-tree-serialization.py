# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-serialization
@Language: Python
@Datetime: 15-07-26 07:37
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""
class Solution:

    '''
    @param root: An object of TreeNode, denote the root of the binary tree.
    This method will be invoked first, you should design your own algorithm 
    to serialize a binary tree which denote by a root node to a string which
    can be easily deserialized by your own "deserialize" method later.
    '''
    def serialize(self, root):
        # write your code here
        res=[]
        q=[]
        
        if root==None:
            return None
        
        q.append(root)
        while len(q)>0 :
            p=q.pop(0)
            if p!=None:
                res.append(str(p.val))
                q.append(p.left)
                q.append(p.right)
            else:
                res.append("#")
            
        return ":".join(res)
        
    '''
    @param data: A string serialized by your serialize method.
    This method will be invoked second, the argument data is what exactly
    you serialized at method "serialize", that means the data is not given by
    system, it's given by your own serialize method. So the format of data is
    designed by yourself, and deserialize it here as you serialize it in 
    "serialize" method.
    '''
    def deserialize(self, data):
        # write your code here
        if data==None:
            return None
            
        res=data.split(':')
        n=len(res)
        root=None
        
        q=[]
        i=0
        root=TreeNode(res[i]) ; i+=1
        q.append(root)
        while len(q)>0 and i<n :
            p=q.pop(0)
            if i<n:
                d=res[i] ; i+=1
                if d !='#':
                    p.left=TreeNode(d)
                    q.append(p.left)
            if i<n:
                d=res[i]; i+=1
                if d != '#':
                    p.right=TreeNode(d)
                    q.append(p.right)
        
        return root
        
            
                
        
                