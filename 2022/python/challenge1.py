from pathlib import Path


def get_data():
    input_path = Path(__file__).parent.parent / "inputs/1-calorie-counting.txt"

    with open(input_path, "r") as file:
        return [line.strip() for line in file]


data = get_data()


def part_one():
    highest_calorie_count = 0
    current_calorie_count = 0
    for line in data:
        if line == "":
            highest_calorie_count = max(highest_calorie_count, current_calorie_count)
            current_calorie_count = 0
            continue
        current_calorie_count += int(line)

    return highest_calorie_count


def part_two():
    highest_calorie_counts = [0, 0, 0]
    current_calorie_count = 0
    for line in data:
        if line == "":
            lowest_high_count = min(highest_calorie_counts)
            lowest_high_count_index = highest_calorie_counts.index(lowest_high_count)
            highest_calorie_counts[lowest_high_count_index] = max(
                lowest_high_count, current_calorie_count
            )
            current_calorie_count = 0
            continue
        current_calorie_count += int(line)

    print(sum(highest_calorie_counts))


print(part_one())
print(part_two())
