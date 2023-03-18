/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 *
 * https://leetcode.cn/problems/find-the-duplicate-number/description/
 *
 * algorithms
 * Medium (64.44%)
 * Likes:    2089
 * Dislikes: 0
 * Total Accepted:    304.3K
 * Total Submissions: 472.8K
 * Testcase Example:  '[1,3,4,2,2]'
 *
 * 给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。
 *
 * 假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。
 *
 * 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,4,2,2]
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,1,3,4,2]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 10^5
 * nums.length == n + 1
 * 1 <= nums[i] <= n
 * nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如何证明 nums 中至少存在一个重复的数字?
 * 你可以设计一个线性级时间复杂度 O(n) 的解决方案吗？
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 快慢指针法
    /// 1. 由于任意的 nums[i] in [1..n], 所以可以定义一种遍历方法：
    ///    对于nums[i], 下一次遍历的下标 next_i = nums[i]
    ///    那么，从起点i开始，可以遍历nums一系列的点，
    ///    这些遍历过程就形成了一个有向链表；
    /// 2. 由于nums.len() == n+1, 故从任何一个元素开始遍历，最后会形成环；
    /// 3. 如果存在 `nums[i] == nums[j] (i != j)`,
    ///    则有 `nums[nums[i]] == nums[nums[j]]`
    ///    后续所有的遍历将合并为一条路径，即两条有向链表必然存在交叉点；
    /// 4. 对于i=0，由于0不在[1..n]范围内，故从nums[0]开始遍历的路径存在一个中间交叉的环，
    ///    遍历路径为6字型；
    /// 5. 找到6字型的交叉点即为重复元素的值；
    /// 6. 为寻找交叉点，可使用快慢指针法进行查找；
    /// 7. 第一步,使用快慢指针, 查找环上的相遇点;
    /// 8. 设0到交叉点的距离为a, 环的长度为b, 相遇时,slow指针移动的距离为s, fast指针移动的距离为f;
    ///    因为fast指针的速度是slow指针的2倍, 所以: `f=2s`.
    ///    另外,fast和slow相遇时, fast和slow的差一定是环长度的正整数倍,
    ///    所以有 `f-s=kb`
    ///    综合以上,可以得到:  `s=kb`
    ///    而从0开始,环的入口为`a+nb`, 相遇时,slow指针已经走了s=kb步, 
    ///    如果s再走a步, 即s+a=a+kb, 就回到环的入口.
    /// 9. 所以,第二步,在快慢指针相遇后, 让fast回到0开始处, slow在相遇点,两者同时按相同的速度遍历
    ///    则当a步后, 两者将在环入口相遇,此时交叉点即为重复值.
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // slow,fast同时从nums[0]开始
        let mut slow = 0;
        let mut fast = 0;

        // 通过快慢指针,计算环上的相遇点
        loop {
            slow = nums[slow] as usize; // s
            fast = nums[nums[fast] as usize] as usize; // f = 2s
            if slow == fast {   //指针相遇, f-s=kb
                break;          //中断遍历
            }
        }

        //将fast重新移动到0开始处, 
        //slow保持在相遇点位置
        fast = 0 as usize; 
        while slow != fast { //两者同步向环相交点移动
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }

        fast as i32  //相交点为重复数
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2]), 2);
    }
}
