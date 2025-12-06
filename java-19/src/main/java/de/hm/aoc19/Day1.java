package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Day1 {
    public static void main(String[] args) throws Exception {
        System.out.println(Paths.get("input/19/day1.txt").toAbsolutePath());
        List<String> lines = Files.readAllLines(Paths.get("input/19/day1.txt"));

        double fuel = lines.stream().mapToDouble(Double::parseDouble).map(i -> Math.floor(i / 3.) - 2).sum();
        System.out.println(fuel);
    }
}
