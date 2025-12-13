import advent2025
import pytest

@pytest.fixture
def ranges_path(tmp_path):
    f = tmp_path / "input.txt"
    input_data = (
        "0:\n"
        "###\n"
        "##.\n"
        "##.\n"
        "\n"
        "1:\n"
        "###\n"
        "##.\n"
        ".##\n"
        "\n"
        "2:\n"
        ".##\n"
        "###\n"
        "##.\n"
        "\n"
        "3:\n"
        "##.\n"
        "###\n"
        "##.\n"
        "\n"
        "4:\n"
        "###\n"
        "#..\n"
        "###\n"
        "\n"
        "5:\n"
        "###\n"
        ".#.\n"
        "###\n"
        "\n"
        "4x4: 0 0 0 0 2 0\n"
        "12x5: 1 0 1 0 2 2\n"
        "12x5: 1 0 1 0 3 2\n"
    )
    f.write_text(input_data)
    return f

def test_py_day12_part1_orig(ranges_path):
    assert advent2025.day12.part1(ranges_path) == 2
