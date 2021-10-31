#!/usr/bin/python3
# -*- coding: UTF-8 -*-
#
# @lc app=leetcode.cn id=1206 lang=python3
#
# [1206] 设计跳表
#
# https://leetcode-cn.com/problems/design-skiplist/description/
#
# algorithms
# Hard (53.80%)
# Likes:    14
# Dislikes: 0
# Total Accepted:    628
# Total Submissions: 1.2K
# Testcase Example:  '["Skiplist","add","add","add","search","add","search","erase","erase","search"]\r\n' +
#  '[[],[1],[2],[3],[0],[4],[1],[0],[1],[1]]\r'
#
# 不使用任何库函数，设计一个跳表。
# 
# 跳表是在 O(log(n))
# 时间内完成增加、删除、搜索操作的数据结构。跳表相比于树堆与红黑树，其功能与性能相当，并且跳表的代码长度相较下更短，其设计思想与链表相似。
# 
# 例如，一个跳表包含 [30, 40, 50, 60, 70, 90]，然后增加 80、45 到跳表中，以下图的方式操作：
# 
# 
# Artyom Kalinin [CC BY-SA 3.0], via Wikimedia Commons
# 
# 跳表中有很多层，每一层是一个短的链表。在第一层的作用下，增加、删除和搜索操作的时间复杂度不超过 O(n)。跳表的每一个操作的平均时间复杂度是
# O(log(n))，空间复杂度是 O(n)。
# 
# 在本题中，你的设计应该要包含这些函数：
# 
# 
# bool search(int target) : 返回target是否存在于跳表中。
# void add(int num): 插入一个元素到跳表。
# bool erase(int num): 在跳表中删除一个值，如果 num 不存在，直接返回false. 如果存在多个 num
# ，删除其中任意一个即可。
# 
# 
# 了解更多 : https://en.wikipedia.org/wiki/Skip_list
# 
# 注意，跳表中可能存在多个相同的值，你的代码需要处理这种情况。
# 
# 样例:
# 
# Skiplist skiplist = new Skiplist();
# 
# skiplist.add(1);
# skiplist.add(2);
# skiplist.add(3);
# skiplist.search(0);   // 返回 false
# skiplist.add(4);
# skiplist.search(1);   // 返回 true
# skiplist.erase(0);    // 返回 false，0 不在跳表中
# skiplist.erase(1);    // 返回 true
# skiplist.search(1);   // 返回 false，1 已被擦除
# 
# 
# 约束条件:
# 
# 
# 0 <= num, target <= 20000
# 最多调用 50000 次 search, add, 以及 erase操作。
# 
# 
#

# @lc code=start


import math
import random

maxLevel = 16
power = 2
maxRand = power ** maxLevel - 1
randLevel = lambda: maxLevel - int(math.log(random.randint(1, maxRand), power))

class SkipNode:
    def __init__(self, val, level = randLevel()):
      self.val = val              #节点值
      self.next = [None] * level  #下一个节点指针数组

    def level(self) -> int:
      return len(self.next)

class Skiplist:
    def __init__(self):
      tail = SkipNode(float('inf'), maxLevel)
      self.head = SkipNode(-float('inf'), maxLevel)
      for i in range(maxLevel):
        self.head.next[i] = tail

    def search(self, target: int) -> bool:
      p = self.head
      i = p.level() - 1
      while p and i >= 0:
        if p.next[i].val == target:
          return True
        elif p.val < target < p.next[i].val:
          i -= 1
        else:
          p = p.next[i]
        
      return False

    def display(self):
      p = self.head
      while p:
        print(str(p.val), len(p.next))
        p = p.next[0]
        
    def add(self, num: int) -> None:
      node = SkipNode(num)
      #从顶部开始遍历各层链表，寻找插入点，将新节点插入其中
      i = node.level() - 1
      p = self.head
      while p and i >= 0:
        if p.val < num < p.next[i].val:
          node.next[i] = p.next[i]
          p.next[i] = node
          i -= 1
        else:
          p = p.next[i]

    def erase(self, num: int) -> bool:
      ans = False
      p = self.head
      i = p.level() - 1
      #for i in range(maxLevel-1, -1, -1):
      while p and i >= 0:
        if p.next[i].val == num:
          ans = True
          p.next[i] = p.next[i].next[i]
          i -= 1
        elif p.val < num < p.next[i].val:
          i -= 1
        else:
          #遍历本层下一个节点
          p = p.next[i]
      return ans

# @lc code=end

# ["Skiplist","add","add","add","add","search","erase","search","search","search"]\r
# ' +
#   '[[],[0],[5],[2],[1],[0],[5],[2],[3],[2]]\r

if __name__ == "__main__":
  sl = Skiplist()
  sl.add(0)
  sl.add(5)
  sl.add(2)
  sl.add(1)
  sl.display()
  print("search 0: ", sl.search(0) )
  sl.erase(5)
  sl.display()
  print("search 2: ", sl.search(2) )
  print("search 3: ", sl.search(3) )
  print("search 2: ", sl.search(2) )
  # sl.erase(6)
  # sl.display()
  # skiplist.erase(0);    // 返回 false，0 不在跳表中
  # skiplist.erase(1);    // 返回 true
  print("search 2: ", sl.search(2) )
  print("search 9: ", sl.search(9) )
    
        
# Your Skiplist object will be instantiated and called as such:
# obj = Skiplist()
# param_1 = obj.search(target)
# obj.add(num)
# param_3 = obj.erase(num)

