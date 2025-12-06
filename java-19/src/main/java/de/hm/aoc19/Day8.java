package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;

public class Day8 {
    public static void main(String[] args) throws Exception {
        String input = new String(Files.readAllBytes(Paths.get("input/19/day8.txt"))).trim();

        int width = 25;
        int height = 6;

        int n = 0;
        List<Layer> layers = new LinkedList<>();
        while (n < input.length()) {
            Layer layer = new Layer(width * height);
            layers.add(layer);
            int p = 0;
            for (int w = 0; w < width; w += 1) {
                for (int h = 0; h < height; h += 1) {
                    int digit = Integer.parseInt(input.substring(n++, n));
                    layer.pixels[p++] = digit;
                    layer.digitCounters[digit] += 1;
                }
            }
        }

        System.out.println("layers: " + layers.size());

        int result = layers.stream().sorted((l1, l2) -> l1.digitCounters[0] - l2.digitCounters[0]).findFirst()
                .map(l -> l.digitCounters[1] * l.digitCounters[2]).get();

        System.out.println("result: " + result);
    }

    static class Layer {
        final int[] pixels;
        final int[] digitCounters;

        public Layer(int pixelCount) {
            pixels = new int[pixelCount];
            digitCounters = new int[10];
        }
    }
}
