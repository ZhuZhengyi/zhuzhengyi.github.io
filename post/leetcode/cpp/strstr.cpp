/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/strstr
@Language: C++
@Datetime: 15-07-20 13:39
*/

class Solution {
public:
    /**
     * Returns a index to the first occurrence of target in source,
     * or -1  if target is not part of source.
     * @param source string to be scanned.
     * @param target string containing the sequence of characters to match.
     */
    int strStr(const char *source, const char *target) {
        // write your code here
        if (source==nullptr || target==nullptr)
            return -1;
        
        if (*source=='\0' && *target=='\0')
            return 0;
        
        for(const char* s0=source; *s0!='\0'; ++s0 ){
            const char* t1=target;
            const char* s1=s0; 
            while((*s1!='\0')&&(*t1!='\0')&&(*s1==*t1)){ ++s1;++t1;}
            if(*t1=='\0'){
                return (s0-source);
            }
        }
        
        return -1;
    }
};

