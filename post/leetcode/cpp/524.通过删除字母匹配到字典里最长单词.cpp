/*
 * @lc app=leetcode.cn id=524 lang=cpp
 *
 * [524] 通过删除字母匹配到字典里最长单词
 *
 * https://leetcode-cn.com/problems/longest-word-in-dictionary-through-deleting/description/
 *
 * algorithms
 * Medium (44.88%)
 * Likes:    57
 * Dislikes: 0
 * Total Accepted:    10.5K
 * Total Submissions: 23.1K
 * Testcase Example:  '"abpcplea"\n["ale","apple","monkey","plea"]'
 *
 * 
 * 给定一个字符串和一个字符串字典，找到字典里面最长的字符串，该字符串可以通过删除给定字符串的某些字符来得到。如果答案不止一个，返回长度最长且字典顺序最小的字符串。如果答案不存在，则返回空字符串。
 * 
 * 示例 1:
 * 
 * 
 * 输入:
 * s = "abpcplea", d = ["ale","apple","monkey","plea"]
 * 
 * 输出: 
 * "apple"
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入:
 * s = "abpcplea", d = ["a","b","c"]
 * 
 * 输出: 
 * "a"
 * 
 * 
 * 说明:
 * 
 * 
 * 所有输入的字符串只包含小写字母。
 * 字典的大小不会超过 1000。
 * 所有输入的字符串长度不会超过 1000。
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    string findLongestWord(string s, vector<string>& d) {
        std::vector<std::string> res;

        for(const auto &el: d) {
            int k = 0;    //
            size_t cnt = 0;  //count for equal chars
            for(size_t i=0; i<el.size(); i++) {
                for(size_t j=k; j<s.size(); j++) {
                    if(s[j] == el[i]) {
                        cnt++;
                        k = j+1;
                        break;
                    }
                }
            }
            if (cnt == el.size())
            {
                res.push_back(el);
            }
        }


        if(res.empty()) {
            return std::string();
        }

        if(res.size() == 1) {
            return res[0];
        }
        
        std::sort(res.begin(), res.end(), [](const std::string &a, const std::string &b){
            return a.size() > b.size();
        });

        std::vector<std::string> res2;
        for(size_t i = 0; i < res.size() -1; i++) {
            if(res[i].size() != res[i+1].size()) {
                res2.push_back(res[i]);
                break;
            } else {
                res2.push_back(res[i]);
                res2.push_back(res[i+1]);
            }
        }

        std::sort(res2.begin(), res2.end());

        return res2[0];
    }
};
// @lc code=end

