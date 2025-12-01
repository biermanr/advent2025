import advent2025
import pytest

@pytest.fixture
def rotations_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "L68\n"
        "L30\n"
        "R48\n"
        "L5\n"
        "R60\n"
        "L55\n"
        "L1\n"
        "L99\n"
        "R14\n"
        "L82\n"
    )
    f.write_text(input_data)
    return f

def test_py_day1_part1(rotations_path):
    assert advent2025.day1.part1(rotations_path) == 3

def test_py_day1_part2(rotations_path):
    assert advent2025.day1.part2(rotations_path) == 6
