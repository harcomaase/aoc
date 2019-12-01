package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Day1b {
    public static void main(String[] args) throws Exception {
        System.out.println(Paths.get("input/19/day1.txt").toAbsolutePath());
        List<String> lines = Files.readAllLines(Paths.get("input/19/day1.txt"));

        int sum = 0;
        for (String line : lines) {
            int mass = Integer.parseInt(line);
            int fuel = calculateFuel(mass);
            while (fuel >= 0) {
                sum += fuel;
                fuel = calculateFuel(fuel);
            }
        }

        System.out.println(sum);
    }

    private static int calculateFuel(int mass) {
        return (int) Math.floor((double) mass / 3.) - 2;
    }
}
