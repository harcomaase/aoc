package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Day12 {
    public static void main(String[] args) throws Exception {
        Pattern pat = Pattern.compile("<x=([0-9-]+), y=([0-9-]+), z=([0-9-]+)>");
        List<Moon> moons = Files
                .readAllLines(Paths.get("input/19/day12_.txt")).stream().map(line -> pat.matcher(line))
                .filter(Matcher::matches).map(m -> new Vector(Integer.parseInt(m.group(1)),
                        Integer.parseInt(m.group(2)), Integer.parseInt(m.group(3))))
                .map(v -> new Moon(v)).collect(Collectors.toList());

        long steps = 1000;
        for (long step = 1; step < steps; step += 1) {
            for (Moon a : moons) {
                for (Moon b : moons) {
                    if (a == b) {
                        continue;
                    }
                    a.vel.x += Integer.compare(b.pos.x, a.pos.x);
                    a.vel.y += Integer.compare(b.pos.y, a.pos.y);
                    a.vel.z += Integer.compare(b.pos.z, a.pos.z);
                }
            }
            for (Moon m : moons) {
                m.pos.x += m.vel.x;
                m.pos.y += m.vel.y;
                m.pos.z += m.vel.z;
            }

        }

        printEnergyOverview(moons);
    }

    static void printEnergyOverview(List<Moon> moons) {
        int total = 0;
        for (Moon moon : moons) {
            int moonTotal = moon.calcTotal();
            total += moonTotal;
            System.out.println("pot=" + moon.calculatePotentialEnergy() + ", kin=" + moon.calculateKineticEnergy()
                    + ", total=" + moonTotal);
        }
        System.out.println("sum of total energy: " + total);
    }

    static class Moon {
        Vector pos;
        Vector vel;

        Moon(Vector pos) {
            this.pos = pos;
            this.vel = new Vector(0, 0, 0);
        }

        int calculatePotentialEnergy() {
            return pos.absSum();
        }

        int calculateKineticEnergy() {
            return vel.absSum();
        }

        int calcTotal() {
            return calculatePotentialEnergy() * calculateKineticEnergy();
        }

        @Override
        public String toString() {
            return "post=" + pos + ", vel=" + vel;
        }
    }

    static class Vector {
        int x;
        int y;
        int z;

        Vector(int x, int y, int z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        int absSum() {
            return Math.abs(x) + Math.abs(y) + Math.abs(z);
        }

        @Override
        public String toString() {
            return "<x=" + x + ", y=" + y + ", z=" + z + ">";
        }
    }
}
