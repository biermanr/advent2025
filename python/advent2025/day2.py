import pathlib

def part1(f_path: pathlib.Path) -> int:
    with open(f_path, 'r') as f:
        ranges = f.read().strip()

    score = 0
    for r in ranges.split(','):
        start,end = r.split('-')
        start = int(start)
        end = int(end)

        for num in range(start, end+1):
            num_str = str(num)
            if len(num_str)%2 == 1:
                continue

            mid = len(num_str)//2
            if num_str[:mid] == num_str[mid:]:
                score += num

    return score

def part2(f_path: pathlib.Path) -> int:
    with open(f_path, 'r') as f:
        ranges = f.read().strip()

    score = 0
    for r in ranges.split(','):
        start,end = r.split('-')
        start = int(start)
        end = int(end)

        for num in range(start, end+1):
            num_str = str(num)

            for substr_len in range(1, len(num_str)//2 + 1):
                for repeat_num in range(1, len(num_str)//substr_len+1):
                    substr = num_str[0:substr_len]*repeat_num
                    if substr == num_str:
                        score += num
                        break
                else:
                    continue
                break

    return score