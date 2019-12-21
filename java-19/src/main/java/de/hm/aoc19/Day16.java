package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;

public class Day16 {
    public static void main(String[] args) throws Exception {
        String line = new String(Files.readAllBytes(Paths.get("input/19/day16.txt"))).trim();

        int[] digits = new int[line.length()];
        int n = 0;
        for (char c : line.toCharArray()) {
            digits[n++] = Character.getNumericValue(c);
        }
        calc(digits, 100);
    }

    static void calc(int[] digits, int phases) {
        int[] currentPhase = Arrays.copyOf(digits, digits.length);
        int[] nextPhase = new int[currentPhase.length];

        int[] basePattern = new int[] { 0, 1, 0, -1 };
        int[] pattern = new int[digits.length];

        for (int phase = 0; phase < phases; phase += 1) {
            for (int outputIndex = 0; outputIndex < currentPhase.length; outputIndex += 1) {
                fillPatternForRound(outputIndex + 1, pattern, basePattern);
                int sum = 0;
                for (int inputIndex = outputIndex; inputIndex < currentPhase.length; inputIndex += 1) {
                    sum += currentPhase[inputIndex] * pattern[inputIndex];
                }
                nextPhase[outputIndex] = Math.abs(sum) % 10;
            }

            for (int i = 0; i < currentPhase.length; i += 1) {
                currentPhase[i] = nextPhase[i];
            }
        }

        System.out.print("After " + phases + "phases : ");
        for (int i = 0; i < Math.min(8, currentPhase.length); i += 1) {
            System.out.print(currentPhase[i]);
        }
        System.out.println();
    }

    static void fillPatternForRound(int round, int[] pattern, int[] basePattern) {
        int basePatternIndex = 0;
        for (int i = 1; i < pattern.length + 2; i += 1) {
            if (i != 1) {
                pattern[i - 2] = basePattern[basePatternIndex % basePattern.length];
            }
            if (i % round == 0) {
                basePatternIndex += 1;
            }
        }
    }
}
