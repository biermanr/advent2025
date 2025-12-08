import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "162,817,812\n"
        "57,618,57\n"
        "906,360,560\n"
        "592,479,940\n"
        "352,342,300\n"
        "466,668,158\n"
        "542,29,236\n"
        "431,825,988\n"
        "739,650,466\n"
        "52,470,668\n"
        "216,146,977\n"
        "819,987,18\n"
        "117,168,530\n"
        "805,96,715\n"
        "346,949,466\n"
        "970,615,88\n"
        "941,993,340\n"
        "862,61,35\n"
        "984,92,344\n"
        "425,690,689\n"
    )
    f.write_text(input_data)
    return f

def test_py_day8_part1(ranges_path):
    assert advent2025.day8.part1(ranges_path) == 40

def test_py_day8_part2(ranges_path):
    assert advent2025.day8.part2(ranges_path) == 25272