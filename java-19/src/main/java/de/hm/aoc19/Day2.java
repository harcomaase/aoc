package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Day2 {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day2.txt"));

        List<Integer> input = Arrays.asList(lines.get(0).split(",")).stream().map(Integer::parseInt).collect(Collectors.toList());

        input.set(1, 12);
        input.set(2, 2);

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

        System.out.println("result: " + input.get(0));
    }
}
