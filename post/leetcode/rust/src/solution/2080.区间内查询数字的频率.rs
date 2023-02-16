/*
 * @lc app=leetcode.cn id=2080 lang=rust
 *
 * [2080] 区间内查询数字的频率
 *
 * https://leetcode-cn.com/problems/range-frequency-queries/description/
 *
 * algorithms
 * Medium (26.63%)
 * Likes:    27
 * Dislikes: 0
 * Total Accepted:    5K
 * Total Submissions: 18.7K
 * Testcase Example:  '["RangeFreqQuery","query","query"]\n' +
  '[[[12,33,4,56,22,2,34,33,22,12,34,56]],[1,2,4],[0,11,33]]'
 *
 * 请你设计一个数据结构，它能求出给定子数组内一个给定值的 频率 。
 * 
 * 子数组中一个值的 频率 指的是这个子数组中这个值的出现次数。
 * 
 * 请你实现 RangeFreqQuery 类：
 * 
 * 
 * RangeFreqQuery(int[] arr) 用下标从 0 开始的整数数组 arr 构造一个类的实例。
 * int query(int left, int right, int value) 返回子数组 arr[left...right] 中 value 的
 * 频率 。
 * 
 * 
 * 一个 子数组 指的是数组中一段连续的元素。arr[left...right] 指的是 nums 中包含下标 left 和 right 在内
 * 的中间一段连续元素。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：
 * ["RangeFreqQuery", "query", "query"]
 * [[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
 * 输出：
 * [null, 1, 2]
 * 
 * 解释：
 * RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2,
 * 34, 33, 22, 12, 34, 56]);
 * rangeFreqQuery.query(1, 2, 4); // 返回 1 。4 在子数组 [33, 4] 中出现 1 次。
 * rangeFreqQuery.query(0, 11, 33); // 返回 2 。33 在整个子数组中出现 2 次。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= arr.length <= 10^5
 * 1 <= arr[i], value <= 10^4
 * 0 <= left <= right < arr.length
 * 调用 query 不超过 10^5 次。
 * 
 * 
 */


// @lc code=start

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        RangeFreqQuery{arr}
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if right <= left {
            return -1;
        }
        let mut right = right;
        if right > self.arr.len() as i32 {
            right = self.arr.len() as i32;
        }

        self.arr[(left as usize)..(right as usize)]
            .iter()
            .filter(|&a| a == &value)
            .count()
            as i32
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
struct RangeFreqQuery {
    arr: Vec<i32>
}
// @lc code=end

