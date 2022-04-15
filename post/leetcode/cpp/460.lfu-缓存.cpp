/*
 * @lc app=leetcode.cn id=460 lang=cpp
 *
 * [460] LFU 缓存
 *
 * https://leetcode-cn.com/problems/lfu-cache/description/
 *
 * algorithms
 * Hard (43.44%)
 * Likes:    481
 * Dislikes: 0
 * Total Accepted:    37.9K
 * Total Submissions: 87.2K
 * Testcase Example:  '["LFUCache","put","put","get","put","get","get","put","get","get","get"]\n' +
  '[[2],[1,1],[2,2],[1],[3,3],[2],[3],[4,4],[1],[3],[4]]'
 *
 * 请你为 最不经常使用（LFU）缓存算法设计并实现数据结构。
 * 
 * 实现 LFUCache 类：
 * 
 * 
 * LFUCache(int capacity) - 用数据结构的容量 capacity 初始化对象
 * int get(int key) - 如果键 key 存在于缓存中，则获取键的值，否则返回 -1 。
 * void put(int key, int value) - 如果键 key 已存在，则变更其值；如果键不存在，请插入键值对。当缓存达到其容量
 * capacity 时，则应该在插入新项之前，移除最不经常使用的项。在此问题中，当存在平局（即两个或更多个键具有相同使用频率）时，应该去除 最近最久未使用
 * 的键。
 * 
 * 
 * 为了确定最不常使用的键，可以为缓存中的每个键维护一个 使用计数器 。使用计数最小的键是最久未使用的键。
 * 
 * 当一个键首次插入到缓存中时，它的使用计数器被设置为 1 (由于 put 操作)。对缓存中的键执行 get 或 put 操作，使用计数器的值将会递增。
 * 
 * 函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。
 * 
 * 
 * 
 * 示例：
 * 
 * 
 * 输入：
 * ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get",
 * "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
 * 输出：
 * [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
 * 
 * 解释：
 * // cnt(x) = 键 x 的使用计数
 * // cache=[] 将显示最后一次使用的顺序（最左边的元素是最近的）
 * LFUCache lfu = new LFUCache(2);
 * lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
 * lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
 * lfu.get(1);      // 返回 1
 * ⁠                // cache=[1,2], cnt(2)=1, cnt(1)=2
 * lfu.put(3, 3);   // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
 * ⁠                // cache=[3,1], cnt(3)=1, cnt(1)=2
 * lfu.get(2);      // 返回 -1（未找到）
 * lfu.get(3);      // 返回 3
 * ⁠                // cache=[3,1], cnt(3)=2, cnt(1)=2
 * lfu.put(4, 4);   // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
 * ⁠                // cache=[4,3], cnt(4)=1, cnt(3)=2
 * lfu.get(1);      // 返回 -1（未找到）
 * lfu.get(3);      // 返回 3
 * ⁠                // cache=[3,4], cnt(4)=1, cnt(3)=3
 * lfu.get(4);      // 返回 4
 * ⁠                // cache=[3,4], cnt(4)=2, cnt(3)=3
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= capacity <= 10^4
 * 0 <= key <= 10^5
 * 0 <= value <= 10^9
 * 最多调用 2 * 10^5 次 get 和 put 方法
 * 
 * 
 */

// @lc code=start
/*
## 解题思路
使用2个数据结构:
* map:   key->(val, list_node_ptr)
* list: (key, freqs)
*/

typedef 
class LFUCache {
    int capacity;   // cache size
    unordered_map<int, pair<int, list<pair<int, int>>::iterator>> kv; // key->(val, (key, freq)::iter)
    list<pair<int, int>> kf;   //(key, freq) order

public:
    LFUCache(int capacity): capacity(capacity) {
    }

    /**
     * @brief adjust kf iter by freq 
     * 
     */
    void adjust(list<pair<int, int>>::iterator iter) {

        auto i = iter;
        i++;
        for(; i!=kf.end(); i++ ) {
            if(i->second < iter->second) {
                swap(*iter, *i);
            }
        }
    }
    
    /*
    1. 根据key，从kv中得到val；
    2. 调整kt
    */
    int get(int key) {
        // key在kv中不存在， 则返回-1
        if (kv.find(key) == kv.end()) {
            return -1;
        }
        //
        auto v_iter = kv[key];
        int res = v_iter.first;
        //调整kf顺序
        auto kf_iter = v_iter.second;
        (kf_iter->second)++;
        //
        adjust(kf_iter);

        return res;
    }
    
    void put(int key, int value) {
        //
        if (kv.find(key) != kv.end()) {
            kv[key].first = value;

            auto kf_iter = kv[key].second;

            //freq reset 1
            kf_iter->second = 1;

            //
            adjust(kf_iter);

            return;
        }

        // full, evict
        if (kv.size() == capacity) {
            kv.erase(kf.front().first);
            kf.pop_front();
        }

        // insert
        kf.push_front(make_pair(key, 1));

        kv[key] = make_pair(value, kf.begin());
    }
};

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @lc code=end

