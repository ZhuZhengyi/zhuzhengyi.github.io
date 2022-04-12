/*
 * @lc app=leetcode.cn id=146 lang=cpp
 *
 * [146] LRU 缓存
 *
 * https://leetcode-cn.com/problems/lru-cache/description/
 *
 * algorithms
 * Medium (52.60%)
 * Likes:    2071
 * Dislikes: 0
 * Total Accepted:    320.9K
 * Total Submissions: 609.7K
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
/*
## 解题思路
1. 主要用到了两个数据结构：
    * map： key -> (val, list_node_ptr)
    * list: (key) -> 
3. get()时，先在map中通过key找到(val, list_node_ptr), 再调整list顺序；
4. put() full evict时，先从list取出tail node淘汰，
*/
class LRUCache {
    int capacity;   //缓存容量

    //unordered_map<int, list<pair<int, int>>::iterator> keys; //key->(val, list_node::iter)
    //list<pair<int,int>> sorted_datas;  //sorted data  //(key1)->(key2)
    list<int> ordered_keys;
    unordered_map<int, pair<int, list<int>::iterator>> kv;

public:
    LRUCache(int capacity): 
        capacity(capacity) {
    }
    
    int get(int key) {
        // keys中不存在，则返回-1
        auto v = kv.find(key);
        if (v == kv.end()) {
            return -1;
        }
        
        // 将keys[key]指向的元素移动到list最前面
        ordered_keys.splice(ordered_keys.begin(), ordered_keys, v->second);

        return v->first;
    }
    
    //
    void put(int key, int value) {
        // 已存在的key
        auto v = kv.find(key);
        if (v != kv.end()) {
            v->first = value;
            ordered_keys.splice(ordered_keys.begin(), ordered_keys, v->second);
            return;
        } else if (ordered_keys.size() == capacity) { //容量满了
            // 移除list中尾部的key和map中对应的;
            kv.erase(ordered_keys.back());
            //
            ordered_keys.pop_back();
        }

        // 将k-v插入到list头
        ordered_keys.push_front(key);
        // 然后将头部指针放入到map中
        kv[key] = make_pair(val, ordered_keys.begin());
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @lc code=end

