package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;

public class Day10b {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day10.txt"));

        List<Asteroid> asteroids = new LinkedList<>();
        for (int y = 0; y < lines.size(); y += 1) {
            String line = lines.get(y);
            for (int x = 0; x < line.length(); x += 1) {
                char c = line.charAt(x);
                if (c == '#') {
                    asteroids.add(new Asteroid(x, y));
                }
            }
        }

        for (Asteroid a : asteroids) {
            getAsteroidsInSight(a, asteroids);
        }

        Asteroid base = asteroids.stream().sorted((a, b) -> b.asteroidsInSight - a.asteroidsInSight).findFirst().get();

        System.out.println("base @ (" + base.x + "|" + base.y + ")");

        int vaporized = 0;
        while (true) {
            // collect all vectors (base -> asteroids)
            List<Vector> vectors = new LinkedList<>();
            // asteroids needs to be visible ones
            for (Asteroid a : getAsteroidsInSight(base, asteroids)) {
                if (a == base) {
                    continue;
                }
                Vector vec = new Vector(a.x - base.x, a.y - base.y, a);
                double length = Math.sqrt(vec.x * vec.x + vec.y * vec.y);
                vec.x /= length;
                vec.y /= length;
                vectors.add(vec);
            }

            List<Asteroid> quadrantUpperRight = vectors.stream().filter(v -> v.x >= 0 && v.y <= 0)
                    .sorted((a, b) -> Double.compare(a.y, b.y)).map(v -> v.a).collect(Collectors.toList());
            List<Asteroid> quadrantLowerRight = vectors.stream().filter(v -> v.x >= 0 && v.y > 0)
                    .sorted((a, b) -> Double.compare(b.x, a.x)).map(v -> v.a).collect(Collectors.toList());
            List<Asteroid> quadrantLowerLeft = vectors.stream().filter(v -> v.x < 0 && v.y > 0)
                    .sorted((a, b) -> Double.compare(b.y, a.y)).map(v -> v.a).collect(Collectors.toList());
            List<Asteroid> quadrantUpperLeft = vectors.stream().filter(v -> v.x < 0 && v.y <= 0)
                    .sorted((a, b) -> Double.compare(a.x, b.x)).map(v -> v.a).collect(Collectors.toList());

            List<List<Asteroid>> arr = new LinkedList<>();
            arr.add(quadrantUpperRight);
            arr.add(quadrantLowerRight);
            arr.add(quadrantLowerLeft);
            arr.add(quadrantUpperLeft);

            for (List<Asteroid> list : arr) {
                for (Asteroid a : list) {
                    asteroids.remove(a);
                    System.out.println("vaporizing (" + a.x + "|" + a.y + ")");
                    vaporized += 1;
                    if (vaporized == 200) {
                        System.out.println("200th: (" + a.x + "|" + a.y + ") -> " + (a.x * 100 + a.y));
                        return;
                    }
                }
            }
        }

    }

    private static List<Asteroid> getAsteroidsInSight(Asteroid a, List<Asteroid> asteroids) {
        List<Asteroid> result = new LinkedList<>();
        for (Asteroid b : asteroids) {
            if (a == b) {
                continue;
            }

            // y = f(x) = mx + n
            double vecX = b.x - a.x;
            double vecY = b.y - a.y;

            boolean sightBlocked = false;
            for (Asteroid c : asteroids) {
                if (a == c || b == c) {
                    continue;
                }

                if (Math.min(a.y, b.y) > c.y || Math.max(a.y, b.y) < c.y || Math.min(a.x, b.x) > c.x
                        || Math.max(a.x, b.x) < c.x) {
                    continue;
                }

                double checkVecX = c.x - a.x;
                double checkVecY = c.y - a.y;
                if (vecX == 0) {
                    if (c.x == a.x) {
                        sightBlocked = true;
                        break;
                    }
                    continue;
                }
                if (vecY == 0) {
                    if (c.y == a.y) {
                        sightBlocked = true;
                        break;
                    }
                    continue;
                }
                double ratioX = checkVecX / vecX;
                double ratioY = checkVecY / vecY;
                if (ratioX == ratioY) {
                    sightBlocked = true;
                    break;
                }
            }
            if (!sightBlocked) {
                a.asteroidsInSight += 1;
                result.add(b);
            }
        }
        return result;
    }

    static class Asteroid {
        int x;
        int y;
        int asteroidsInSight;

        public Asteroid(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    static class Vector {
        double x;
        double y;
        Asteroid a;

        public Vector(int x, int y, Asteroid a) {
            this.x = x;
            this.y = y;
            this.a = a;
        }
    }
}
