package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.Date;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Optional;

public class Day18 {

    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("../input/19/day18.txt"));

        char[][] laby = createLabyrinth(lines);
        resetWater(laby);

        Branch branch = new Branch();
        branch.player = getPositionOf('@', laby).get();
        branch.laby = laby;

        List<Branch> branches = iterate(branch);

        System.out.println(new Date());

        System.out.println("found " + branches.size() + " branches");

        System.out.println("shortest: " + branches.stream().mapToInt(b -> b.steps).min().getAsInt());
    }

    private static List<Branch> iterate(Branch branch) throws Exception {
        Map<Position, Integer> reachableKeys = findReachableKeys(branch);
        resetWater(branch.laby);
        if (reachableKeys.isEmpty()) {
            return Collections.singletonList(branch);
        }
        List<Branch> result = new LinkedList<>();
        for (Map.Entry<Position, Integer> entry : reachableKeys.entrySet()) {
            Position pos = entry.getKey();
            int distance = entry.getValue();

            Branch nextBranch = reachableKeys.size() == 1 ? branch : new Branch(branch);
            String door = String.valueOf(nextBranch.laby[pos.y][pos.x]).toUpperCase();
            nextBranch.laby[pos.y][pos.x] = ' ';
            nextBranch.player.x = pos.x;
            nextBranch.player.y = pos.y;
            getPositionOf(door.charAt(0), nextBranch.laby)
                    .ifPresent(doorPos -> nextBranch.laby[doorPos.y][doorPos.x] = ' ');
            nextBranch.steps += distance;
            result.addAll(iterate(nextBranch));
        }
        return result;
    }

    private static Map<Position, Integer> findReachableKeys(Branch branch) {
        List<Position> water = new LinkedList<>();
        water.add(new Position(branch.player.x, branch.player.y));
        branch.laby[branch.player.y][branch.player.x] = '.';
        Map<Position, Integer> reachableKeys = new HashMap<>();

        for (int i = 1;; i += 1) {
            List<Position> nextWater = new LinkedList<>();
            for (Position wat : water) {
                int step = i;
                handleWaterPosition(branch.laby, wat.x, wat.y + 1, nextWater)
                        .ifPresent(pos -> reachableKeys.putIfAbsent(pos, step));
                handleWaterPosition(branch.laby, wat.x, wat.y - 1, nextWater)
                        .ifPresent(pos -> reachableKeys.putIfAbsent(pos, step));
                handleWaterPosition(branch.laby, wat.x + 1, wat.y, nextWater)
                        .ifPresent(pos -> reachableKeys.putIfAbsent(pos, step));
                handleWaterPosition(branch.laby, wat.x - 1, wat.y, nextWater)
                        .ifPresent(pos -> reachableKeys.putIfAbsent(pos, step));
            }
            if (nextWater.isEmpty()) {
                break;
            }
            // water.addAll(nextWater);
            water = nextWater;
        }

        return reachableKeys;
    }

    private static Optional<Position> handleWaterPosition(char[][] laby, int x, int y, List<Position> nextWater) {
        char c = laby[y][x];
        if (c == ' ') {
            nextWater.add(new Position(x, y));
            laby[y][x] = '.';
        } else if (isKey(c)) {
            return Optional.of(new Position(x, y));
        }
        return Optional.empty();
    }

    private static char[][] createLabyrinth(List<String> lines) {
        char[][] laby = new char[lines.size()][lines.get(0).length()];
        int y = 0;
        for (String line : lines) {
            int x = 0;
            for (char c : line.toCharArray()) {
                laby[y][x] = c;
                x += 1;
            }
            y += 1;
        }
        return laby;
    }

    static boolean isKey(char c) {
        return c >= 'a' && c <= 'z';
    }

    static void resetWater(char[][] laby) {
        for (int y = 0; y < laby.length; y += 1) {
            for (int x = 0; x < laby[y].length; x += 1) {
                if (laby[y][x] == '.') {
                    laby[y][x] = ' ';
                }
            }
        }
    }

    static Optional<Position> getPositionOf(char c, char[][] laby) {
        for (int y = 0; y < laby.length; y += 1) {
            for (int x = 0; x < laby[y].length; x += 1) {
                if (laby[y][x] == c) {
                    return Optional.of(new Position(x, y));
                }
            }
        }
        return Optional.empty();
    }

    static void printLabyrinth(char[][] laby) {
        for (int y = 0; y < laby.length; y += 1) {
            for (int x = 0; x < laby[y].length; x += 1) {
                System.out.print(laby[y][x]);
            }
            System.out.println();
        }
    }

    static class Branch {
        int steps;
        Position player;
        char[][] laby;

        Branch() {
        }

        Branch(Branch other) {
            this.steps = other.steps;
            this.player = new Position(other.player.x, other.player.y);
            this.laby = new char[other.laby.length][other.laby[0].length];
            for (int y = 0; y < laby.length; y += 1) {
                for (int x = 0; x < laby[y].length; x += 1) {
                    this.laby[y][x] = other.laby[y][x];
                }
            }
        }

    }

    static class Position {
        int x;
        int y;

        Position(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object obj) {
            if (!(obj instanceof Position)) {
                return false;
            }
            Position o = (Position) obj;
            return this.x == o.x && this.y == o.y;
        }

        @Override
        public int hashCode() {
            return x * 1000 + y;
        }
    }
}
