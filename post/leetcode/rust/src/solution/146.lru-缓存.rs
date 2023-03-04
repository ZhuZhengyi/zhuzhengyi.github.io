/*
* @lc app=leetcode.cn id=146 lang=rust
*
* [146] LRU 缓存
*
* https://leetcode.cn/problems/lru-cache/description/
*
* algorithms
* Medium (53.45%)
* Likes:    2566
* Dislikes: 0
* Total Accepted:    452.6K
* Total Submissions: 846.9K
* Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n' +
 '[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
*
* 请你设计并实现一个满足  LRU (最近最少使用) 缓存 约束的数据结构。
*
* 实现 LRUCache 类：
*
*
*
*
* LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存
* int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
* void put(int key, int value) 如果关键字 key 已经存在，则变更其数据值 value ；如果不存在，则向缓存中插入该组
* key-value 。如果插入操作导致关键字数量超过 capacity ，则应该 逐出 最久未使用的关键字。
*
*
* 函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。
*
*
*
*
*
* 示例：
*
*
* 输入
* ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
* [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
* 输出
* [null, null, null, 1, null, -1, null, -1, 3, 4]
*
* 解释
* LRUCache lRUCache = new LRUCache(2);
* lRUCache.put(1, 1); // 缓存是 {1=1}
* lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
* lRUCache.get(1);    // 返回 1
* lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
* lRUCache.get(2);    // 返回 -1 (未找到)
* lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
* lRUCache.get(1);    // 返回 -1 (未找到)
* lRUCache.get(3);    // 返回 3
* lRUCache.get(4);    // 返回 4
*
*
*
*
* 提示：
*
*
* 1 <= capacity <= 3000
* 0 <= key <= 10000
* 0 <= value <= 10^5
* 最多调用 2 * 10^5 次 get 和 put
*
*
*/

// @lc code=start
use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;

/// LRUCache
struct LRUCache {
    keys: VecDeque<i32>,   // keys
    kv: HashMap<i32, i32>, // cache map
    cap: usize,            // cap of caches
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            keys: VecDeque::new(),
            kv: HashMap::new(),
            cap: capacity.try_into().unwrap(),
        }
    }

    /// 1. 判断key是否在map中；
    /// 2. 如果不在map中，则返回-1;
    /// 3. 如果在map中：
    /// 3.1 查找key在sorted_keys中的index；
    /// 3.2 删除sorted_keys中的旧key；
    /// 3.3 将新key插入到sorted_keys头；
    /// 3.4. 返回map中的key->val;
    fn get(&mut self, key: i32) -> i32 {
        match self.kv.get(&key) {
            None => -1,
            Some(&v) => {
                let i = self.keys.iter().position(|&k| k == key).unwrap();
                self.keys.remove(i);
                self.keys.push_front(key);
                v
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.kv.get(&key) {
            None => {
                if self.is_full() {
                    let invalid_key = self.keys.pop_back().unwrap();
                    self.kv.remove(&invalid_key);
                }
                self.kv.insert(key, value); //insert k-v
                self.keys.push_front(key);
            }
            Some(_) => {
                let i = self.keys.iter().position(|&k| k == key).unwrap();
                self.keys.remove(i);
                self.keys.push_front(key);
                self.kv.insert(key, value); //update k-v
            }
        }
    }

    fn is_full(&self) -> bool {
        self.keys.len() >= self.cap
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
