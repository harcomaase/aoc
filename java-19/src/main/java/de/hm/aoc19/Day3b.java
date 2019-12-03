package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;

public class Day3b {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("../input/19/day3.txt"));

        List<List<Point>> paths = new LinkedList<>();
        for (String line : lines) {
            String[] split = line.split(",");
            int x = 0;
            int y = 0;
            List<Point> path = new LinkedList<>();
            for (String mov : split) {
                int steps = Integer.parseInt(mov.substring(1));
                for (int i = 0; i < steps; i += 1) {
                    switch (mov.charAt(0)) {
                    case 'U':
                        y += 1;
                        break;
                    case 'D':
                        y -= 1;
                        break;
                    case 'L':
                        x -= 1;
                        break;
                    case 'R':
                        x += 1;
                        break;
                    }
                    path.add(new Point(x, y));
                }
            }
            System.out.println("adding path with " + path.size() + " points");
            paths.add(path);
        }

        List<Integer> intersectionSteps = new LinkedList<>();
        for (List<Point> path1 : paths) {
            for (List<Point> path2 : paths) {
                if (path1 == path2) {
                    continue;
                }

                int steps1 = 0;
                for (Point p1 : path1) {
                    steps1 += 1;
                    int steps2 = 0;
                    for (Point p2 : path2) {
                        steps2 += 1;
                        if (p1.equals(p2)) {
                            intersectionSteps.add(steps1+steps2);
                        }
                    }
                }
            }
        }

        System.out.println("found " + intersectionSteps.size() + " intersections");

        int leastSteps = intersectionSteps.stream().mapToInt(p -> p).min().orElseThrow();

        System.out.println("leastSteps: " + leastSteps);
    }

    static class Point {
        private final int x;
        private final int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        public int getX() {
            return x;
        }

        public int getY() {
            return y;
        }

        @Override
        public boolean equals(Object obj) {
            if (!(obj instanceof Point)) {
                return false;
            }
            Point other = (Point) obj;
            return this.x == other.x && this.y == other.y;
        }
    }
}
