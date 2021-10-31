# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/lru-cache
@Language: Python
@Datetime: 15-08-25 14:53
'''

class LRUCache:

    # @param capacity, an integer
    def __init__(self, capacity):
        # write your code here
        self.cap=capacity
        self.cacheList=[]
        self.cacheMap={}

    # @return an integer
    def get(self, key):
        # write your code here
        data=self.cacheMap.get(key)
        if data == None:
            return -1
            
        self.cacheList.remove(data)
        self.cacheList.insert(0, data)
        self.cacheMap[key]=data
        
        return data[1]
        
    # @param key, an integer
    # @param value, an integer
    # @return nothing
    def set(self, key, value):
        # write your code here
        data=self.cacheMap.get(key)
        
        if data != None:
            self.cacheList.remove(data)
        else:
            if len(self.cacheList) == self.cap:
                self.cacheMap.pop(self.cacheList[-1][0])
                self.cacheList.pop()
                
        data=[key, value]
        self.cacheList.insert(0,data)
        self.cacheMap[key]=data
