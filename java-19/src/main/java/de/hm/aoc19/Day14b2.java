package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Day14b2 {
    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Paths.get("input/19/day14.txt"));
        List<Reaction> reactions = new LinkedList<>();
        for (String line : lines) {
            String[] split = line.split(" => ");

            Chemical out = new Chemical(split[1].trim());
            List<Chemical> ins = new LinkedList<>();

            for (String rawIn : split[0].split(",")) {
                ins.add(new Chemical(rawIn.trim()));
            }
            reactions.add(new Reaction(out, ins));
        }

        long target = 1000000000000L;
        long actual = 0;

        long fuel = 0;
        for (long l = 3279000;; l += 1) {
            long ore = calculateNeededOre(reactions, l);
            System.out.println(l + " FUEL need " + ore + " ORE");
            if (ore < target) {
                actual = ore;
                fuel = l;
            } else {
                break;
            }

        }
        System.out.println("got " + fuel + " fuel for " + target +  " ORE (" + actual + " actually)");
    }

    static long calculateNeededOre(List<Reaction> reactions, long fuel) {

        long neededOre = 0;
        List<Chemical> tooMuch = new LinkedList<>();
        List<Chemical> need = findFor(fuel, "FUEL", reactions, tooMuch);
        while (true) {
            boolean allOre = true;
            List<Chemical> nextNeed = new LinkedList<>();
            for (Chemical c : need) {
                nextNeed.addAll(findFor(c.amount, c.type, reactions, tooMuch));
            }

            for (Iterator<Chemical> it = nextNeed.iterator(); it.hasNext();) {
                Chemical c = it.next();
                if (c.type.equals("ORE")) {
                    neededOre += c.amount;
                    it.remove();
                } else {
                    allOre = false;
                }
            }

            need = nextNeed;

            if (allOre) {
                break;
            }
        }

        return neededOre;

    }

    private static List<Chemical> findFor(long amount, String type, List<Reaction> reactions,
            List<Chemical> chemsTooMuch) {
        for (Reaction r : reactions) {
            if (r.out.type.equals(type)) {
                long existingTooMuch = findTooMuch(type, chemsTooMuch);
                if (existingTooMuch >= amount) {
                    updateChemsTooMuch(type, -amount, chemsTooMuch);
                    return Collections.emptyList();
                }
                long neededAmount = amount - existingTooMuch;
                updateChemsTooMuch(type, -existingTooMuch, chemsTooMuch);
                long multiplier = r.out.amount < neededAmount
                        ? (long) Math.ceil((double) neededAmount / (double) r.out.amount)
                        : 1;
                long tooMuch = r.out.amount * multiplier - neededAmount;
                updateChemsTooMuch(type, tooMuch, chemsTooMuch);
                return r.ins.stream().map(c -> {
                    Chemical chem = new Chemical(c.amount * multiplier, c.type);
                    return chem;
                }).collect(Collectors.toList());
            }
        }
        throw new IllegalStateException("should not happen");
    }

    private static void updateChemsTooMuch(String type, long existingTooMuch, List<Chemical> chemsTooMuch) {
        for (Chemical c : chemsTooMuch) {
            if (c.type.equals(type)) {
                c.amount += existingTooMuch;
                return;
            }
        }
        chemsTooMuch.add(new Chemical(existingTooMuch, type));
    }

    private static long findTooMuch(String type, List<Chemical> chemsTooMuch) {
        for (Chemical c : chemsTooMuch) {
            if (c.type.equals(type)) {
                return c.amount;
            }
        }
        return 0;
    }

    static class Reaction {
        Chemical out;
        List<Chemical> ins;

        Reaction(Chemical out, List<Chemical> ins) {
            this.out = out;
            this.ins = ins;
        }

        @Override
        public String toString() {
            return ins + " => " + out;
        }
    }

    static class Chemical {
        long amount;
        String type;
        static Pattern pat = Pattern.compile("([0-9]+) ([A-Z]+)");

        Chemical(long amount, String type) {
            this.amount = amount;
            this.type = type;
        }

        Chemical(String input) {
            Matcher m = pat.matcher(input);
            m.matches();
            this.amount = Long.parseLong(m.group(1));
            this.type = m.group(2);
        }

        @Override
        public String toString() {
            return amount + " " + type;
        }
    }
}
