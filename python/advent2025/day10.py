import pathlib

from scipy.optimize import linprog
import numpy as np

def parse_line(line: str) -> tuple[np.ndarray, np.ndarray]:
    # [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    tokens = line.strip().split(' ')

    target_joltage = np.array(list(map(int, tokens[-1][1:-1].split(','))))
    b_matrix = []

    for i in range(1, len(tokens)-1):
        idxs = list(map(int, tokens[i][1:-1].split(',')))
        row = [0] * len(target_joltage)
        for idx in idxs:
            row[idx] = 1
        b_matrix.append(np.array(row))

    b_matrix = np.array(b_matrix)

    return b_matrix, target_joltage

def solve_lp(b_matrix: np.ndarray, target_joltage: np.ndarray) -> int:
    num_vars = b_matrix.shape[0]
    c = np.ones(num_vars)

    A_eq = b_matrix.T
    b_eq = target_joltage

    res = linprog(c, A_eq=A_eq, b_eq=b_eq)

    if res.success:
        return int(round(res.x.sum()))
    else:
        raise ValueError("Linear programming failed to find a solution")

def part2(f_path: pathlib.Path) -> int:
    score = 0
    with open(f_path) as f:
        for line in f:
            joltage, buttons = parse_line(line)
            score += solve_lp(joltage, buttons)

    return score