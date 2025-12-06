package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Day2b {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day2.txt"));

        List<Integer> input = Arrays.asList(lines.get(0).split(",")).stream().map(Integer::parseInt)
                .collect(Collectors.toList());

        int result = calculateResult(input);

        System.out.println("result: " + result);
    }

    private static int calculateResult(List<Integer> input) {
        int noun, verb = 0;
        int expected = 19690720;

        for (noun = 0; noun <= 99; noun += 1) {
            for (verb = 0; verb <= 99; verb += 1) {
                if (calculate(input, noun, verb) == expected) {
                    return 100 * noun + verb;
                }
            }
        }

        throw new IllegalStateException("this should not happen");
    }

    private static int calculate(List<Integer> originalInput, int noun, int verb) {
        List<Integer> input = new ArrayList<>(originalInput);

        input.set(1, noun);
        input.set(2, verb);

        for (int i = 0; i < input.size(); i += 4) {
            int opcode = input.get(i);
            if (opcode == 99) {
                break;
            }
            int i1 = input.get(i + 1);
            int i2 = input.get(i + 2);
            int i3 = input.get(i + 3);
            int term1 = input.get(i1);
            int term2 = input.get(i2);
            int result = opcode == 1 ? term1 + term2 : term1 * term2;

            input.set(i3, result);
        }

        return input.get(0);
    }
}
