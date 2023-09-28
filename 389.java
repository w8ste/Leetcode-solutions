class Solution {
    public char findTheDifference(String s, String t) {
       if(s.length() == 0) return t.toCharArray()[0];
       HashMap<Character, Integer> map = new HashMap<>();
       for(char c : s.toCharArray()) {
           if(map.containsKey(c)) {
               map.put(c , map.get(c) + 1);
           
           }
           else {
               map.put(c, 1);
           }
       }
       for(char c : t.toCharArray()) {
           if(!map.containsKey(c)) return c;
           if(map.get(c) - 1 < 0) return c;
           map.put(c, map.get(c) - 1);
       }
       throw new IllegalArgumentException();
    }
}
