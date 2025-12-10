import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n"
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n"
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n"
    )
    f.write_text(input_data)
    return f

def test_py_day10_part2_orig(ranges_path):
    assert advent2025.day10.part2(ranges_path) == 33


@pytest.fixture
def other_test(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "[..#.##] (0,1,3,4) (0,3,4) (0,5) (0,1,2) (3,5) (0,2,3,4) (2,3) {58,27,37,57,37,24}\n"
    )
    f.write_text(input_data)
    return f

def test_py_day10_part2_other(other_test):
    assert advent2025.day10.part2(other_test) == 78