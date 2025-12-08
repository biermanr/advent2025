import pathlib

def closest_n_pairs(boxes: list[list[int]], n: int) -> list[tuple[int, int, int]]:
    dists = []
    for i in range(len(boxes)-1):
        for j in range(i + 1, len(boxes)):
            dist = sum((boxes[i][k] - boxes[j][k]) ** 2 for k in range(3))
            dists.append((dist, i, j))

    if n == None:
        return sorted(dists)
    else:
        return sorted(dists)[:n]


def part1(f_path: pathlib.Path) -> int:
    boxes = []
    with open(f_path) as f:
        for line in f:
            boxes.append(list(map(int, line.strip().split(','))))

    circuits = [set([i]) for i in range(len(boxes))]
    n = 10 if len(boxes) <= 20 else 1000
    pairs = closest_n_pairs(boxes, n)
    for _, i, j in pairs:
        c1_idx = next(idx for idx, c in enumerate(circuits) if i in c)

        if j in circuits[c1_idx]:
            continue

        c1 = circuits[c1_idx]
        del circuits[c1_idx]

        c2_idx = next(idx for idx, c in enumerate(circuits) if j in c)
        c2 = circuits[c2_idx]
        del circuits[c2_idx]

        circuits.append(c1.union(c2))

    circuit_sizes = [len(c) for c in circuits]
    circuit_sizes.sort()

    score = 1
    for size in circuit_sizes[-3:]:
        score *= size

    return score

def part2(f_path: pathlib.Path) -> int:
    boxes = []
    with open(f_path) as f:
        for line in f:
            boxes.append(list(map(int, line.strip().split(','))))

    circuits = [set([i]) for i in range(len(boxes))]
    pairs = closest_n_pairs(boxes, None)

    for _, i, j in pairs:
        c1_idx = next(idx for idx, c in enumerate(circuits) if i in c)

        if j in circuits[c1_idx]:
            continue

        c1 = circuits[c1_idx]
        del circuits[c1_idx]

        c2_idx = next(idx for idx, c in enumerate(circuits) if j in c)
        c2 = circuits[c2_idx]
        del circuits[c2_idx]

        circuits.append(c1.union(c2))
        
        if len(circuits) == 1:
            break

    return boxes[i][0] * boxes[j][0]