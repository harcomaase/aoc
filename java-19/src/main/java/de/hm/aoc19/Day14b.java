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

public class Day14b {
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

        long neededOre = 0;
        long fuel=0;
        List<Chemical> tooMuch = new LinkedList<>();
        List<Chemical> need;
        // System.out.println("need " + need + " for 1 FUEL");
        while (true) {
            need = findFor(1, "FUEL", reactions, tooMuch);
            while (true) {
                // for(int i = 0; i < 3; i+=1) {
                boolean allOre = true;
                List<Chemical> nextNeed = new LinkedList<>();
                for (Chemical c : need) {
                    nextNeed.addAll(findFor(c.amount, c.type, reactions, tooMuch));
                    // System.out.println("need " + need + " for " + c.amount + " " + c.type);
                }

                for (Iterator<Chemical> it = nextNeed.iterator(); it.hasNext();) {
                    Chemical c = it.next();
                    if (c.type.equals("ORE")) {
                        // System.out.println(c.amount + " ORE required");
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
            if(neededOre > 1_000_000_000_000L) {
                break;
            }
            fuel += 1;
            if(fuel % 1000 == 0) {
                System.out.println("now @ " + fuel + " fuel for " + neededOre + " ore");
            }
        }

        // System.out.println("excessive stuff: " + tooMuch);

        System.out.println("max amount of fuel: " + fuel);

    }

    private static List<Chemical> findFor(long amount, String type, List<Reaction> reactions,
            List<Chemical> chemsTooMuch) {
        // System.out.println("for " + amount + " " + type);
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
                    // System.out.println(" - " + chem);
                    return chem;
                }).collect(Collectors.toList());
            }
        }
        throw new IllegalStateException("should not happen");
    }

    private static void updateChemsTooMuch(String type, long existingTooMuch, List<Chemical> chemsTooMuch) {
        // System.out.println("updating too much: " + existingTooMuch + " " + type);
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
