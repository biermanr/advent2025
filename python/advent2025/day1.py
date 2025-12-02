import pathlib

def part1(f_path: pathlib.Path) -> int:
    dial = 50
    dial_max = 99
    dial_span = dial_max + 1
    score = 0

    with open(f_path, 'r') as f:
        for line in f:
            d,m = line[0], int(line[1:])
            m *= -1 if d == 'L' else 1

            dial = (dial + m) % dial_span

            if dial == 0:
                score += 1

    return score

def part2(f_path: pathlib.Path) -> int:
    dial = 50
    dial_max = 99
    dial_span = dial_max + 1
    score = 0

    with open(f_path, 'r') as f:
        for line in f:
            d,m = line[0], int(line[1:])

            # Count how many full spins we make, add to score, then update the magnitude to be less than a full spin
            full_spins = m // dial_span
            score += full_spins
            m -= full_spins * dial_span
            m *= -1 if d == 'L' else 1

            # Check if the remaining movement causes us to pass the zero point in either direction
            # need to be careful for when the dial is already at zero
            new_dial = (dial + m) % dial_span
            if new_dial == 0:
                score += 1
            elif ((d == 'R') and (new_dial < dial) and (dial != 0)):
                score += 1
            elif ((d == 'L') and (new_dial > dial) and (dial != 0)):
                score += 1

            dial = new_dial

    return score