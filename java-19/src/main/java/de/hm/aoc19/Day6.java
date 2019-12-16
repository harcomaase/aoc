package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;

public class Day6 {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day6.txt"));

        Orbit root = new Orbit(null, "COM");
        List<Orbit> orbits = new LinkedList<>();

        orbits.add(root);

        for (String line : lines) {
            String[] x = line.split("\\)");
            orbits.add(new Orbit(x[0], x[1]));
        }

        int total = 0;
        for (Orbit orbit : orbits) {
            total += countOrbits(orbits, orbit);
        }

        System.out.println("total direct and indirect orbits: " + total);
    }

    private static int countOrbits(List<Orbit> orbits, Orbit orbit) {
        Orbit it = orbit;
        int n = 0;
        while (it.getObject() != null) {
            n += 1;
            it = findOrbitedObject(orbits, it);
        }
        return n;
    }

    private static Orbit findOrbitedObject(List<Orbit> orbits, Orbit orbiter) {
        for (Orbit orbited : orbits) {
            if (orbiter.getObject().equals(orbited.getOrbitedBy())) {
                return orbited;
            }
        }
        throw new IllegalStateException("no orbited object found for " + orbiter.getObject() + "!");
    }

    static class Orbit {
        private final String object;
        private final String orbitedBy;

        public Orbit(String object, String orbitedBy) {
            this.object = object;
            this.orbitedBy = orbitedBy;
        }

        public String getObject() {
            return object;
        }

        public String getOrbitedBy() {
            return orbitedBy;
        }
    }
}
