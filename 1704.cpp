#include <bits/stdc++.h> 
class Solution {
public:
    bool halvesAreAlike(string s) {
        //alternative solution which converts every character to lower case 
        //for(auto& x: s) {
        //    x = tolower(x);
        //}
        int m = s.size() / 2;
        int c1 = 0, c2 = 0;
        for(int i = 0, j = m; i < m && j < s.size(); j++, i++) {
            if(isVowel(s[i])) c1++;
            if(isVowel(s[j])) c2++;
        
        }
        return c1 == c2;
    }

    bool isVowel(char c) {
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' ||
        c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'; //im the alternative solution, we would not need this line
    }
};
