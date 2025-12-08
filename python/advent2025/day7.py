import pathlib

def find_start_position(grid: list[list[str]]) -> tuple[int, int]:
    for row_idx in range(len(grid)):
        for col_idx in range(len(grid[0])):
            if grid[row_idx][col_idx] == 'S':
                return (row_idx, col_idx)

def traverse(grid: list[list[str]], memo: dict[tuple[int, int], int], row: int, col: int) -> int:
    if row >= len(grid) - 1:
        return 1

    if grid[row][col] == '^':
        if (row, col) in memo:
            return memo[(row, col)]
        else:
            left = traverse(grid, memo, row + 1, col - 1) if col > 0 else 0
            right = traverse(grid, memo, row + 1, col + 1) if col < len(grid[0]) - 1 else 0
            memo[(row, col)] = left + right
            return left + right
    
    return traverse(grid, memo, row + 1, col)

def part1(f_path: pathlib.Path) -> int:
    grid = []
    with open(f_path) as f:
        for line in f:
            grid.append(line.strip())

    memo = {}
    start_row, start_col = find_start_position(grid)
    traverse(grid, memo, start_row, start_col)

    return len(memo)

def part2(f_path: pathlib.Path) -> int:
    grid = []
    with open(f_path) as f:
        for line in f:
            grid.append(line.strip())

    memo = {}
    start_row, start_col = find_start_position(grid)
    return traverse(grid, memo, start_row, start_col)