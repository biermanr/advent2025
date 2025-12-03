import pathlib
import functools

def part1(f_path: pathlib.Path) -> int:
    with open(f_path) as f_in:
        battery_packs = [[int(c) for c in line.strip()] for line in f_in.readlines()]

    score_func = functools.partial(process_battery_pack, nth=2)
    return sum(map(score_func, battery_packs))

def part2(f_path: pathlib.Path) -> int:
    with open(f_path) as f_in:
        battery_packs = [[int(c) for c in line.strip()] for line in f_in.readlines()]

    score_func = functools.partial(process_battery_pack, nth=12)
    return sum(map(score_func, battery_packs))

def process_battery_pack(battery_pack: list[int], nth: int) -> int:
    if nth == 0 or len(battery_pack) == 0:
        return 0

    max_battery = 0
    max_battery_index = 0
    for index in range(len(battery_pack)-nth+1):
        if battery_pack[index] > max_battery:
            max_battery = battery_pack[index]
            max_battery_index = index

    score = max_battery*10**(nth-1)
    return score+process_battery_pack(battery_pack[max_battery_index+1:], nth-1)