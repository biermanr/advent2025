import pathlib
import numpy as np
from typing import TypeAlias
import functools

Present: TypeAlias = tuple[tuple[int, ...], ...]
Problem: TypeAlias = tuple[int, int, list[int]]

def create(lines: list[str]) -> Present:
    nrows = len(lines)
    ncols = len(lines[0])

    grid = [[0]*ncols for _ in range(nrows)]
    for i in range(nrows):
        for j in range(ncols):
            if lines[i][j] == '#':
                grid[i][j] = 1

    for i in range(nrows):
        grid[i] = tuple(grid[i])

    return tuple(grid)

def num_cells(p: Present) -> int:
    return sum([sum(r) for r in p])

def from_np(p: np.ndarray) -> Present:
    return tuple([tuple(r) for r in p])

def show(p: Present) -> None:
    for r in p:
        print(r)

def combine(p1: Present, p2: Present) -> Present:
    max_h = max(len(p1), len(p2))
    max_w = max(len(p1[0]), len(p2[0]))

    combined_p = np.zeros((max_h, max_w), dtype=int)

    for y in range(max_h):
        for x in range(max_w):
            v1 = p1[y][x] if (y < len(p1)) and (x < len(p1[0])) else 0
            v2 = p2[y][x] if (y < len(p2)) and (x < len(p2[0])) else 0

            if v1+v2 == 2:
                return None # presents overlap, can't combine
            elif v1 == 1:
                combined_p[y][x] = 1
            elif v2 == 1:
                combined_p[y][x] = 1
            else:
                combined_p[y][x] = 0

    return from_np(combined_p)

def shift_down(p: Present, n: int) -> Present:
    # Add n leading rows of 0's
    offset_present = [tuple([0]*len(p[0])) for _ in range(n)]
    for r in p:
        offset_present.append(r)

    return tuple(offset_present)

def shift_right(p: Present, n: int) -> Present:
    # Add n leading 0's to each row
    offset_present = []
    for r in p:
        offset_present.append(tuple([0]*n+list(r)))

    return tuple(offset_present)


def all_rotations(p: Present) -> set[Present]:
    array_p = np.array(p)
    rotations = set()

    for k in range(4):
        rotations.add(from_np(np.rot90(array_p, k)))

    return rotations

def all_flips(p: Present) -> set[Present]:
    flips = set()
    flips.add(p)

    array_p = np.array(p)

    flips.add(from_np(np.fliplr(array_p)))
    flips.add(from_np(np.flipud(array_p)))

    return flips


def all_configurations(p: Present) -> set[Present]:
    configs = set([p])

    rot_configs = all_rotations(p)
    configs.update(rot_configs)

    for rp in rot_configs:
        configs.update(all_flips(rp))

    return configs


def parse_input(f_path: pathlib.Path) -> tuple[list[set[Present]], list[Problem]]:
    presents = []
    problem_lines = []

    with open(f_path) as f_in:
        current_shape_lines = []
        for line in f_in:
            line = line.strip()
            if not line:
                # Blank line
                continue

            elif ':' in line and 'x' not in line:
                # Header line for new shape
                if current_shape_lines:
                    presents.append(create(current_shape_lines))

                current_shape_lines = []

            elif 'x' in line:
                # Counts of required presents line
                problem_lines.append(line)

            else:
                current_shape_lines.append(line)

    # Add the final present
    if current_shape_lines:
        presents.append(create(current_shape_lines))

    # Process the problems
    problems = []
    for problem_line in problem_lines:
        sizes, requirements = problem_line.split(": ")
        w,h = map(int, sizes.split('x'))
        requirements = list(map(int, requirements.split(' ')))
        problems.append((w, h, requirements))

    return presents, problems

@functools.cache
def _helper(p1_conf: Present, p2_conf: Present) -> Present:
    # Arbitrarily move p1_conf around. Would be smarter to always move the larger present
    w2, h2 = len(p2_conf[0]), len(p2_conf)

    smallest_area = None
    best_present = None

    for col_offset in range(h2+1):
        for row_offset in range(w2+1):
            shifted_p1_conf = shift_down(shift_right(p1_conf, row_offset), col_offset)
            combined_p = combine(shifted_p1_conf, p2_conf)
            if combined_p:
                cw, ch = len(combined_p[0]), len(combined_p)
                if not smallest_area or cw*ch < smallest_area:
                    best_present = combined_p
                    smallest_area = cw*ch

    return best_present

def best_paired_arrangement(p1: Present, p2: Present, d1: int, d2: int) -> Present:
    min_area = None
    best_present = None

    for p1_conf in all_configurations(p1):
        for p2_conf in all_configurations(p2):
            joined_present = _helper(p1_conf, p2_conf)
            w, h = len(joined_present[0]), len(joined_present)
            fits = (w <= d1 and h <= d2) or (w <= d2 and h <= d1)
            if not fits:
                continue

            if not min_area or w*h < min_area:
                best_present = joined_present
                min_area = w*h

    return best_present

def recur_helper(state, presents_to_add, d1, d2) -> bool:

    # Base case, the present layout is too big
    if not state:
        return False

    # Base case, we've added all the presents
    if len(presents_to_add) == 0:
        return True

    for i,p in enumerate(presents_to_add):

        # Recurse one time: Update state and remove present from consideration
        recur_result = recur_helper(
            best_paired_arrangement(state, p, d1, d2),
            [q for j,q in enumerate(presents_to_add) if j != i],
            d1, d2
        )

        if recur_result:
            return True

    return False


    
def solve_problem(presents, problem) -> bool:
    h, w, num_presents = problem
    d1, d2 = min(h,w), max(h,w) # d1 is the "minor" and d2 "major" dimension
    presents_to_add = []
    for i,n in enumerate(num_presents):
        for _ in range(n):
            presents_to_add.append(presents[i])

    state = ((0, 0, 0), (0, 0, 0), (0, 0, 0)) #all shapes are 3x3 so we start with an empty board

    return recur_helper(state, presents_to_add, d1, d2)
    

def part1(f_path: pathlib.Path) -> int:
    presents, problems = parse_input(f_path)

    score = 0

    for problem in problems:
        
        #placeable = solve_problem(presents, problem)
        w,h,num_presents = problem
        grid_area = w*h

        total_present_area = 0
        for i,n in enumerate(num_presents):
            p = presents[i]
            total_present_area += n * num_cells(p)

        print(f"Total present area {total_present_area} vs grid area {grid_area}")
        if total_present_area < grid_area:
            score += 1

    return score

