package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

public class Day6b {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day6.txt"));

        // define santa as center of mass
        Map<String, List<String>> orbitMap = new HashMap<>();

        for (String line : lines) {
            String[] x = line.split("\\)");
            addOrbit(orbitMap, x[0], x[1]);
            addOrbit(orbitMap, x[1], x[0]);
        }

        int steps = findWay(orbitMap, "YOU",null, -1);

        System.out.println("required orbital transfers: " + steps);
    }

    private static int findWay(Map<String, List<String>> orbitMap, String o, String before, int steps) {
        List<String> l = orbitMap.get(o);
        if (l.contains("SAN")) {
            return steps;
        }
        for (String ooo : l) {
            if (ooo.equals(before)) {
                continue;
            }
            int n = findWay(orbitMap, ooo, o, steps + 1);
            if (n > 0) {
                return n;
            }
        }
        return 0;
    }

    private static void addOrbit(Map<String, List<String>> orbitMap, String o1, String o2) {
        List<String> l = orbitMap.get(o1);
        if (l == null) {
            l = new LinkedList<>();
            orbitMap.put(o1, l);
        }
        l.add(o2);
    }

}
