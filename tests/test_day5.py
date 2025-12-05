import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "3-5\n"
        "10-14\n"
        "16-20\n"
        "12-18\n"
        "\n"
        "1\n"
        "5\n"
        "8\n"
        "11\n"
        "17\n"
        "32\n"
    )
    f.write_text(input_data)
    return f

def test_py_day5_part1(ranges_path):
    assert advent2025.day5.part1(ranges_path) == 3

def test_py_day5_part2(ranges_path):
    assert advent2025.day5.part2(ranges_path) == 14