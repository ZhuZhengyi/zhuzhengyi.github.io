/*
 * @lc app=leetcode.cn id=1488 lang=rust
 *
 * [1488] 避免洪水泛滥
 *
 * https://leetcode.cn/problems/avoid-flood-in-the-city/description/
 *
 * algorithms
 * Medium (26.32%)
 * Likes:    124
 * Dislikes: 0
 * Total Accepted:    12.4K
 * Total Submissions: 47.2K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 你的国家有无数个湖泊，所有湖泊一开始都是空的。当第 n 个湖泊下雨前是空的，那么它就会装满水。如果第 n 个湖泊下雨前是 满的 ，这个湖泊会发生 洪水
 * 。你的目标是避免任意一个湖泊发生洪水。
 *
 * 给你一个整数数组 rains ，其中：
 *
 *
 * rains[i] > 0 表示第 i 天时，第 rains[i] 个湖泊会下雨。
 * rains[i] == 0 表示第 i 天没有湖泊会下雨，你可以选择 一个 湖泊并 抽干 这个湖泊的水。
 *
 *
 * 请返回一个数组 ans ，满足：
 *
 *
 * ans.length == rains.length
 * 如果 rains[i] > 0 ，那么ans[i] == -1 。
 * 如果 rains[i] == 0 ，ans[i] 是你第 i 天选择抽干的湖泊。
 *
 *
 * 如果有多种可行解，请返回它们中的 任意一个 。如果没办法阻止洪水，请返回一个 空的数组 。
 *
 * 请注意，如果你选择抽干一个装满水的湖泊，它会变成一个空的湖泊。但如果你选择抽干一个空的湖泊，那么将无事发生。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：rains = [1,2,3,4]
 * 输出：[-1,-1,-1,-1]
 * 解释：第一天后，装满水的湖泊包括 [1]
 * 第二天后，装满水的湖泊包括 [1,2]
 * 第三天后，装满水的湖泊包括 [1,2,3]
 * 第四天后，装满水的湖泊包括 [1,2,3,4]
 * 没有哪一天你可以抽干任何湖泊的水，也没有湖泊会发生洪水。
 *
 *
 * 示例 2：
 *
 *
 * 输入：rains = [1,2,0,0,2,1]
 * 输出：[-1,-1,2,1,-1,-1]
 * 解释：第一天后，装满水的湖泊包括 [1]
 * 第二天后，装满水的湖泊包括 [1,2]
 * 第三天后，我们抽干湖泊 2 。所以剩下装满水的湖泊包括 [1]
 * 第四天后，我们抽干湖泊 1 。所以暂时没有装满水的湖泊了。
 * 第五天后，装满水的湖泊包括 [2]。
 * 第六天后，装满水的湖泊包括 [1,2]。
 * 可以看出，这个方案下不会有洪水发生。同时， [-1,-1,1,2,-1,-1] 也是另一个可行的没有洪水的方案。
 *
 *
 * 示例 3：
 *
 *
 * 输入：rains = [1,2,0,1,2]
 * 输出：[]
 * 解释：第二天后，装满水的湖泊包括 [1,2]。我们可以在第三天抽干一个湖泊的水。
 * 但第三天后，湖泊 1 和 2 都会再次下雨，所以不管我们第三天抽干哪个湖泊的水，另一个湖泊都会发生洪水。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= rains.length <= 10^5
 * 0 <= rains[i] <= 10^9
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - hashmap + btreeset
    /// 1. 使用hashmap  rain_dates记录每个湖下雨的日期;
    ///    btreeset dry_dates记录可以排水的日期(rains[i] == 0);
    /// 2. 依次检查每天的下雨记录;
    /// 3. 如果未下雨(rains[i] == 0), 则将当天日期i记录到dry_dates中;
    /// 4. 否则, 说明当天lake = rains[i]下雨了;
    /// 5. 此时先通过rain_dates检查该lake以前是否下过雨;
    /// 6. 如果该lake之前下过, 则在day_dates中检查之前下雨后到当前时间前是否有可以排水的适当日期;
    /// 6.1. 如果没有,则湖水将溢出泛滥, 返回空;
    /// 6.2. 否则有, 则在该日期排出之前的湖水, 并更新;
    /// 7. 更新rain_dates中的记录;
    /// 8. 设置当天不排水;
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeSet;
        use std::collections::HashMap;
        use std::ops::Bound::*;

        let mut dry = vec![1; rains.len()]; //记录排水计划
        let mut dry_dates = BTreeSet::new(); //可排水的日期
        let mut rain_dates = HashMap::new(); //湖的下雨日期

        // 检查每天下雨的记录
        for (day, &lake) in rains.iter().enumerate() {
            // 未下雨
            if lake == 0 {
                dry_dates.insert(day); //记录当前可排水的日期
                continue;
            }

            // 否则,当天下雨了

            // 如果该湖之前下过雨
            if rain_dates.contains_key(&lake) {
                // 如果存在可供排水的适当日期(在之前湖下雨后, 今天之前)
                if let Some(old_day) = dry_dates
                    .range((Excluded(rain_dates[&lake]), Excluded(day))) //该湖前一次下雨日期之后, 今天之前
                    .next() // 存在可排水的日子
                    .copied()
                {
                    dry[old_day] = lake; //在可排水的日子j时, 抽干湖中的水
                    dry_dates.remove(&old_day); //移除已排过水的日期记录
                } else {
                    // 湖里已经有水且没有适合排水的日期, 则发生洪水
                    return vec![];
                }
            }

            rain_dates.insert(lake, day); //记录已下过雨的lake
            dry[day] = -1; // 当前天不用抽水
        }

        dry
    }
}
// @lc code=end

struct Solution;
