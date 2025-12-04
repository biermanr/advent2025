import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "..@@.@@@@.\n"
        "@@@.@.@.@@\n"
        "@@@@@.@.@@\n"
        "@.@@@@..@.\n"
        "@@.@@@@.@@\n"
        ".@@@@@@@.@\n"
        ".@.@.@.@@@\n"
        "@.@@@.@@@@\n"
        ".@@@@@@@@.\n"
        "@.@.@@@.@.\n"
    )
    f.write_text(input_data)
    return f

def test_py_day4_part1(ranges_path):
    assert advent2025.day4.part1(ranges_path) == 13

def test_py_day4_part2(ranges_path):
    assert advent2025.day4.part2(ranges_path) == 43