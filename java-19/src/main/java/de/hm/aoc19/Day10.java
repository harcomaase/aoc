package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;

public class Day10 {
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

                    if (Math.min(a.y, b.y) > c.y || Math.max(a.y, b.y) < c.y || Math.min(a.x, b.x) > c.x || Math.max(a.x, b.x) < c.x) {
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
                }
            }
        }

        asteroids.stream().sorted((a, b) -> b.asteroidsInSight - a.asteroidsInSight)
                .forEach(a -> System.out.println(a.asteroidsInSight));
        Asteroid best = asteroids.stream().sorted((a, b) -> b.asteroidsInSight - a.asteroidsInSight).findFirst().get();

        System.out.println("best asteroid is with " + best.asteroidsInSight + " asteroids in sight");
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
}
