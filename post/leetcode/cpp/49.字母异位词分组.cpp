/*
 * @lc app=leetcode.cn id=49 lang=cpp
 *
 * [49] 字母异位词分组
 *
 * https://leetcode-cn.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (67.20%)
 * Likes:    1117
 * Dislikes: 0
 * Total Accepted:    318.3K
 * Total Submissions: 473.5K
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
 * 
 * 字母异位词 是由重新排列源单词的字母得到的一个新单词，所有源单词中的字母通常恰好只用一次。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
 * 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * 
 * 示例 2:
 * 
 * 
 * 输入: strs = [""]
 * 输出: [[""]]
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: strs = ["a"]
 * 输出: [["a"]]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] 仅包含小写字母
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /**
    * ## 解题思路
    * * hashmap
    * 1. 遍历strs，使用hashmap来记录遍历中遇到的每个str；
    * 2. hashmap的key为sort(str), val为[str]；
    * 3. 遍历完strs后，遍历hashmap,获取所有的val；
    */
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> m;
        
        // 统计各个string 排序后的 map
        for(auto str: strs) {
            string s = str;
            sort(s.begin(), s.end());
            m[s].push_back(str);
        }

        vector<vector<string>> res;
        for(auto mi: m) {
            res.push_back(mi.second);
        }
        return res;
    }
};


// @lc code=end

