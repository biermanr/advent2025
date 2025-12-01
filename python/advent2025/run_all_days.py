from advent2025 import advent2025
import argparse
import time
import contextlib

# stolen from Henry https://github.com/henryiii/aoc2025/blob/main/python/problems.py
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
        (1, [advent2025.day1_part1, advent2025.day1_part2]),
    ]

    for day, funcs in days:
        for part,func in enumerate(funcs):
            with timer():
                result = func(f"{args.data}/day{day}.txt")
                print(f"Day {day} part {part} result {result}", end=" ")

if __name__ == "__main__":
    main()
