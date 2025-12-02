import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    f.write_text(input_data)
    return f

def test_py_day1_part1(ranges_path):
    assert advent2025.day2.part1(ranges_path) == 1227775554

def test_py_day1_part2(ranges_path):
    assert advent2025.day2.part2(ranges_path) == 4174379265