import pathlib

def part1(f_path: pathlib.Path) -> int:
    fresh_ranges = []
    score = 0
    with open(f_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            if '-' in line:
                str_s, str_e = line.split('-')
                s, e = int(str_s), int(str_e)
                fresh_ranges.append((s, e))

            else:
                val = int(line)
                for s, e in fresh_ranges:
                    if s <= val <= e:
                        score += 1
                        break

    return score

def part2(f_path: pathlib.Path) -> int:
    fresh_ranges = []
    score = 0
    with open(f_path) as f:
        for line in f:
            line = line.strip()
            if '-' not in line:
                continue

            str_s, str_e = line.split('-')
            s, e = int(str_s), int(str_e)
            fresh_ranges.append((s, e))


    fresh_ranges.sort()
    prev_s, prev_e = fresh_ranges[0]

    for s, e in fresh_ranges[1:]:
        if s <= prev_e:
            prev_e = max(prev_e, e)
        else:
            score += prev_e - prev_s + 1
            prev_s, prev_e = s, e

    score += prev_e - prev_s + 1

    return score