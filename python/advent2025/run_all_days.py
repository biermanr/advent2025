import advent2025 as py_advent2025
from advent2025 import advent2025 as rs_advent2025
import argparse
import time
import contextlib

@contextlib.contextmanager
def timer():
    start = time.perf_counter()
    yield
    print(f"({(time.perf_counter() - start)*1000:.2f}ms)")

def main():
    parser = argparse.ArgumentParser(description="Run all days of Advent of Code 2025")
    parser.add_argument("--data", type=str, default="data", help="Folder containing input data files like data/ where it expects to find data/day1.txt data/day2.txt etc.")
    args = parser.parse_args()

    days = [
        (1, "rust", [rs_advent2025.day1_part1, rs_advent2025.day1_part2]),
        (1, "python", [py_advent2025.day1.part1, py_advent2025.day1.part2]),
        (2, "rust", [rs_advent2025.day2_part1, rs_advent2025.day2_part2]),
        (2, "python", [py_advent2025.day2.part1, py_advent2025.day2.part2]),
        (3, "rust", [rs_advent2025.day3_part1, rs_advent2025.day3_part2]),
        (3, "python", [py_advent2025.day3.part1, py_advent2025.day3.part2]),
        (4, "rust", [rs_advent2025.day4_part1, rs_advent2025.day4_part2]),
        (4, "python", [py_advent2025.day4.part1, py_advent2025.day4.part2]),
        (5, "rust", [rs_advent2025.day5_part1, rs_advent2025.day5_part2]),
        (5, "python", [py_advent2025.day5.part1, py_advent2025.day5.part2]),
        (6, "rust", [rs_advent2025.day6_part1, rs_advent2025.day6_part2]),
        (6, "python", [py_advent2025.day6.part1, py_advent2025.day6.part2]),
        (7, "rust", [rs_advent2025.day7_part1, rs_advent2025.day7_part2]),
    ]

    for day, language, funcs in days:
        for part,func in enumerate(funcs):
            with timer():
                result = func(f"{args.data}/day{day}.txt")
                print(f"Day {day} part {part+1} result {result} using language {language}", end=" ")

if __name__ == "__main__":
    main()
