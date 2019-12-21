package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;

public class Day16b {
    public static void main(String[] args) throws Exception {
        String line = new String(Files.readAllBytes(Paths.get("input/19/day16.txt"))).trim();

        int[] digits = new int[line.length() * 10000];
        int n = 0;
        for (int i = 0; i < 10000; i += 1) {
            for (char c : line.toCharArray()) {
                digits[n++] = Character.getNumericValue(c);
            }
        }
        calc(digits, 100, Integer.parseInt(line.substring(0, 7)));
    }

    static void calc(int[] digits, int phases, int offset) {
        int[] currentPhase = Arrays.copyOf(digits, digits.length);
        int[] nextPhase = new int[digits.length];

        System.out.println("offset at " + offset + " of " + digits.length);

        for (int phase = 0; phase < phases; phase += 1) {
            int sum = 0;
            for (int outputIndex = currentPhase.length - 1; outputIndex >= offset; outputIndex -= 1) {
                sum += currentPhase[outputIndex];
                nextPhase[outputIndex] = Math.abs(sum) % 10;
            }

            for (int i = 0; i < currentPhase.length; i += 1) {
                currentPhase[i] = nextPhase[i];
            }

            System.out.println("completed phase " + phase + " calc");
        }

        System.out.print("After " + phases + "phases : ");
        for (int i = offset; i <= offset + 8; i += 1) {
            System.out.print(currentPhase[i]);
        }
        System.out.println();
    }
}
