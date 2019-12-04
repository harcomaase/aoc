package de.hm.aoc19;

import java.util.LinkedList;
import java.util.List;

public class Day4b {
    public static void main(String[] args) throws Exception {

        long start = 146810;
        long end = 612564;

        List<Long> passwords = new LinkedList<>();
        for (long i = start; i <= end; i += 1) {
            String value = Long.toString(i);

            // check 2 adjacent digits
            char lastDigit = value.charAt(0);
            boolean adjacentValid = false;
            boolean increasingValid = true;
            for (int n = 1; n < 6; n += 1) {
                char adjacentDigit = value.charAt(n);
                if (!adjacentValid && lastDigit == adjacentDigit) {
                    if ((n == 1 || value.charAt(n - 2) != lastDigit) && (n == 5 || value.charAt(n + 1) != lastDigit)) {
                        adjacentValid = true;
                    }
                }
                if (lastDigit > adjacentDigit) {
                    increasingValid = false;
                    break;
                }
                lastDigit = adjacentDigit;
            }

            if (adjacentValid && increasingValid) {
                passwords.add(i);
            }
        }

        System.out.println("number of possible passwords: " + passwords.size());
    }
}
