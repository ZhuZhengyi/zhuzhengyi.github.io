/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 *
 * https://leetcode-cn.com/problems/find-median-from-data-stream/description/
 *
 * algorithms
 * Hard (52.43%)
 * Likes:    657
 * Dislikes: 0
 * Total Accepted:    77.5K
 * Total Submissions: 147.8K
 * Testcase Example:  '["MedianFinder","addNum","addNum","findMedian","addNum","findMedian"]\n' +
  '[[],[1],[2],[],[3],[]]'
 *
 * 中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。
 * 
 * 例如，
 * 
 * [2,3,4] 的中位数是 3
 * 
 * [2,3] 的中位数是 (2 + 3) / 2 = 2.5
 * 
 * 设计一个支持以下两种操作的数据结构：
 * 
 * 
 * void addNum(int num) - 从数据流中添加一个整数到数据结构中。
 * double findMedian() - 返回目前所有元素的中位数。
 * 
 * 
 * 示例：
 * 
 * addNum(1)
 * addNum(2)
 * findMedian() -> 1.5
 * addNum(3) 
 * findMedian() -> 2
 * 
 * 进阶:
 * 
 * 
 * 如果数据流中所有整数都在 0 到 100 范围内，你将如何优化你的算法？
 * 如果数据流中 99% 的整数都在 0 到 100 范围内，你将如何优化你的算法？
 * 
 * 
 */

// @lc code=start
use std::collections::BinaryHeap;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /// ## 解题思路
    /// * 双堆法
    fn new() -> Self {
        MedianFinder{
            count: 0,
            upper: BinaryHeap::new(),
            lower: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.count+=1;
        self.upper.push(-num);
        self.lower.push(self.upper.pop().unwrap_or(0_i32) * -1) ;
        if self.count % 2 != 0 {
            self.upper.push(self.lower.pop().unwrap_or(0_i32) * -1);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.count % 2 != 0 { 
            (*(self.upper.peek().unwrap_or(&0_i32)) as f64) * -1_f64
        } else {
            (*(self.upper.peek().unwrap_or(&0_i32)) as f64 * -1_f64 + *(self.lower.peek().unwrap_or(&0_i32)) as f64) / 2_f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
struct MedianFinder {
    upper: BinaryHeap<i32>,
    lower: BinaryHeap<i32>,
    count: u64,
}
// @lc code=end

