import java.util.stream.*;

class Solution {
    public String reverseWords(String s) {
        return Stream.of(s.split(" ")).map((e) -> {
        return new StringBuilder(e).reverse().toString() + " ";
        }).collect(Collectors.joining()).trim();
    }
}
