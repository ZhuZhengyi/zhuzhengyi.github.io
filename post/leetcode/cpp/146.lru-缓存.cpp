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
1. 包含get(), put()两个方法；
2. cache有一定的capacity;
3. 使用map来存储key到value，以满足get()的常数需求；
4. 使用list来存储按访问顺序排列的；
5. get(), put()操作通过调整list，来满足least recently需求；
*/
class LRUCache {
    int capacity;   //缓存容量
    unordered_map<int, list<pair<int, int>>::iterator> keys; //keys
    list<pair<int,int>> sorted_datas;  //sorted data

public:
    LRUCache(int capacity): 
        capacity(capacity) {
    }
    
    int get(int key) {
        // keys中不存在，则返回-1
        if (keys.find(key) == keys.end()) {
            return -1;
        }
        
        // 将keys[key]指向的元素移动到list最前面
        sorted_datas.splice(sorted_datas.begin(), sorted_datas, keys[key]);

        return keys[key]->second;
    }
    
    //
    void put(int key, int value) {
        // 已存在的key
        if (keys.find(key) != keys.end()) {
            keys[key]->second = value;
            sorted_datas.splice(sorted_datas.begin(), sorted_datas, keys[key]);
            return;
        } else if (sorted_datas.size() == capacity) { //容量满了
            // 移除list中尾部的key和map中对应的;
            keys.erase(sorted_datas.back().first);
            //
            sorted_datas.pop_back();
        }

        // 将k-v插入到list头
        sorted_datas.push_front({key, value});
        // 然后将头部指针放入到map中
        keys[key] = sorted_datas.begin();
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @lc code=end

