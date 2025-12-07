import pathlib
import functools
import operator

def part1(f_path: pathlib.Path) -> int:
    grid = []
    with open(f_path) as f:
        for line in f:
            grid.append(line.strip().split())

    ops = grid.pop()  # Last line contains operations

    num_problems = len(ops)
    nums_per_problem = len(grid)

    score = 0
    for problem_idx in range(num_problems):
        nums = [int(grid[num_idx][problem_idx]) for num_idx in range(nums_per_problem)]
        score += functools.reduce(
            operator.mul if ops[problem_idx] == '*' else operator.add, nums
        )

    return score

def part2(f_path: pathlib.Path) -> int:
    grid = []
    with open(f_path) as f:
        for line in f:
            grid.append(line)

    ops = grid.pop()
    num_digits_per_problem = len(grid)

    problem_starts = []
    for i,o in enumerate(ops):
        if o == '*' or o == '+':
            problem_starts.append(i)

    problem_ends = problem_starts[1:] + [len(ops)]

    problems = []
    for start, end in zip(problem_starts, problem_ends):
        problem_nums = []
        for i in range(start, end-1):
            digits = []
            for d in range(num_digits_per_problem):
                if grid[d][i] != ' ':
                    digits.append(grid[d][i])

            problem_nums.append(int(''.join(digits)))

        problems.append(problem_nums)

    score = 0
    for problem_idx, nums in enumerate(problems):
        score += functools.reduce(
            operator.mul if ops[problem_starts[problem_idx]] == '*' else operator.add, nums
        )

    return score