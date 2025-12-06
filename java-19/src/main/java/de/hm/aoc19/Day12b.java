package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Day12b {
    public static void main(String[] args) throws Exception {
        Pattern pat = Pattern.compile("<x=([0-9-]+), y=([0-9-]+), z=([0-9-]+)>");
        List<Moon> moons = Files
                .readAllLines(Paths.get("input/19/day12.txt")).stream().map(line -> pat.matcher(line))
                .filter(Matcher::matches).map(m -> new Vector(Integer.parseInt(m.group(1)),
                        Integer.parseInt(m.group(2)), Integer.parseInt(m.group(3))))
                .map(v -> new Moon(v)).collect(Collectors.toList());

        int[] startStateX = new int[moons.size() * 2];
        int[] startStateY = new int[moons.size() * 2];
        int[] startStateZ = new int[moons.size() * 2];
        calculateStates(moons, startStateX, startStateY, startStateZ);

        int[] currentStateX = new int[moons.size() * 2];
        int[] currentStateY = new int[moons.size() * 2];
        int[] currentStateZ = new int[moons.size() * 2];

        long[] steps = new long[] { 0, 0, 0 };
        for (long step = 1;; step += 1) {
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

            calculateStates(moons, currentStateX, currentStateY, currentStateZ);
            if (steps[0] == 0) {
                boolean match = true;
                for (int i = 0; i < currentStateX.length; i += 1) {
                    match &= currentStateX[i] == startStateX[i];
                }
                if (match) {
                    System.out.println("found match for X axis at " + step + " steps");
                    steps[0] = step;
                }
            }
            if (steps[1] == 0) {
                boolean match = true;
                for (int i = 0; i < currentStateY.length; i += 1) {
                    match &= currentStateY[i] == startStateY[i];
                }
                if (match) {
                    System.out.println("found match for Y axis at " + step + " steps");
                    steps[1] = step;
                }
            }
            if (steps[2] == 0) {
                boolean match = true;
                for (int i = 0; i < currentStateZ.length; i += 1) {
                    match &= currentStateZ[i] == startStateZ[i];
                }
                if (match) {
                    System.out.println("found match for Z axis at " + step + " steps");
                    steps[2] = step;
                }
            }

            if (steps[0] != 0 && steps[1] != 0 && steps[2] != 0) {
                System.out.println("YES! At step " + lcm(lcm(steps[0],steps[1]),steps[2]));
                break;
            }

            if (step % 1_000_000 == 0) {
                System.out.println("did " + step + " calcs");
            }
        }
    }

    private static long lcm(long a, long b) {
        return a * (b / gcd(a, b));
    }

    private static long gcd(long a, long b) {
        while (b > 0) {
            long temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    private static void calculateStates(List<Moon> moons, int[] stateX, int[] stateY, int[] stateZ) {
        int n = 0;
        for (Moon m : moons) {
            stateX[n] = m.pos.x;
            stateY[n] = m.pos.y;
            stateZ[n] = m.pos.z;
            n += 1;
            stateX[n] = m.vel.x;
            stateY[n] = m.vel.y;
            stateZ[n] = m.vel.z;
            n += 1;
        }
    }

    static int getTotalEnergy(List<Moon> moons) {
        int total = 0;
        for (Moon moon : moons) {
            total += moon.calcTotal();
        }
        return total;
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
