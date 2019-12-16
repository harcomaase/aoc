package de.hm.aoc19;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.LinkedList;
import java.util.List;

public class Day8b {
    public static void main(String[] args) throws Exception {
        String input = new String(Files.readAllBytes(Paths.get("input/19/day8.txt"))).trim();

        int width = 6;
        int height = 25;

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

        Layer picture = new Layer(width * height);

        n = 0;
        for (int w = 0; w < width; w += 1) {
            for (int h = 0; h < height; h += 1) {
                int pixel = 2;
                for (Layer layer : layers) {
                    int lp = layer.pixels[n];
                    if (lp != 2) {
                        pixel = lp;
                        break;
                    }
                }
                picture.pixels[n] = pixel;
                n += 1;
            }
        }

        System.out.println("picture:");
        n = 0;
        for (int w = 0; w < width; w += 1) {
            for (int h = 0; h < height; h += 1) {
                int p = picture.pixels[n++];
                System.out.print(p == 1 ?"#" : " ");
            }
            System.out.println();
        }

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
