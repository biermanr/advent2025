import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "987654321111111\n"
        "811111111111119\n"
        "234234234234278\n"
        "818181911112111\n"
    )
    f.write_text(input_data)
    return f

def test_py_day3_part1(ranges_path):
    assert advent2025.day3.part1(ranges_path) == 357

def test_py_day3_part2(ranges_path):
    assert advent2025.day3.part2(ranges_path) == 3121910778619