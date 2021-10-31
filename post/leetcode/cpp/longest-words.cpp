/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-words
@Language: C++
@Datetime: 15-07-19 03:57
*/

class Solution {
public:
    /**
     * @param dictionary: a vector of strings
     * @return: a vector of strings
     */
    vector<string> longestWords(vector<string> &dictionary) {
        // write your code here
        size_t max_len=0;
        const int size=dictionary.size();
        vector<size_t> lens;
        
        for(int i=0; i<size; ++i){
            auto l=dictionary[i].size();
            lens.push_back(l);
            max_len=max(l, max_len);
        }
        
        vector<string> strs;
        for(int i=0; i<size; ++i){
            if(lens[i]==max_len ){
                strs.push_back(dictionary[i]);
            }
        }
        return strs;
        
    }
};
