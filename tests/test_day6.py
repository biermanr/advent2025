import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "123 328  51 64 \n"
        " 45 64  387 23 \n" 
        "  6 98  215 314\n"
        "*   +   *   +  \n"
    )
    f.write_text(input_data)
    return f

def test_py_day6_part1(ranges_path):
    assert advent2025.day6.part1(ranges_path) == 4277556

def test_py_day6_part2(ranges_path):
    assert advent2025.day6.part2(ranges_path) == 3263827