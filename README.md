# Hexdiff

A simple binary comparison tool.

This program simply compares two input files and highlights differing bytes between them. It does not utilize any fancy diffing algorithms, it simply compares bytes at the same offset. If the files are of differing length, it will pad the shorter one with zeroes.
