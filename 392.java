class Solution {
    public boolean isSubsequence(String s, String t) {
        int count = 0;
        for (int i = 0; i < t.length(); i++) {
            if(s.length() <= count) break;
            else if(s.charAt(count) == t.charAt(i)) count++;
        }
        return s.length() == count;
        
    }
}
