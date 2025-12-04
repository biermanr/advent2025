import pathlib
import functools

def part1(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        data = [list(line.strip()) for line in f.readlines()]

    score = 0
    for y in range(len(data)):
        for x in range(len(data[0])):
            if data[y][x] == '@':
                adjacent = count_adjacent(data, x, y)
                if adjacent < 4:
                    score += 1

    return score

def part2(f_path: pathlib.Path) -> int:
    with open(f_path) as f:
        data = [list(line.strip()) for line in f.readlines()]

    score = 0
    while True:
        removable_positions = []

        for y in range(len(data)):
            for x in range(len(data[0])):
                if data[y][x] == '@':
                    adjacent = count_adjacent(data, x, y)
                    if adjacent < 4:
                        score += 1
                        removable_positions.append((x, y))

        if not removable_positions:
            break

        for x, y in removable_positions:
            data[y][x] = '.'

    return score

# This is AI generated, super nice
def count_adjacent(diagram: list[list[str]], x: int, y: int) -> int:
    deltas = [(-1, -1), (-1, 0), (-1, 1),
              (0, -1),          (0, 1),
              (1, -1),  (1, 0), (1, 1)]
    count = 0
    for dx, dy in deltas:
        nx, ny = x + dx, y + dy
        if 0 <= nx < len(diagram) and 0 <= ny < len(diagram[0]):
            if diagram[ny][nx] == '@':
                count += 1
    return count