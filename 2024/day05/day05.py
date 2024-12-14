


import re
import sys


class PageUpdate:
    def __init__(self, value, all_rules):
        self.value = value
        self.all_rules = all_rules

    def __lt__(self, other):
        for rule in self.all_rules:
            if self.value == rule.num1 and other.value == rule.num2:
                return True
            if self.value == rule.num2 and other.value == rule.num1:
                return False

        return True

    def __repr__(self):
        return f"{self.value}"

class Rule:
    def __init__(self, num1, num2):
        self.num1 = num1
        self.num2 = num2

    def __repr__(self):
        return f"Rule({self.num1}, {self.num2})"
    
    def validate_update_list(self, update_list):
        for index in range(0, len(update_list)):
            if update_list[index].value == self.num1:
                for possible_invalid_update in update_list[:index]:
                    if possible_invalid_update.value == self.num2:
                        return False

        return True

def add_middle_values(update_lists):
    middle_update_sum = 0
    for update_list in update_lists:
        middle_number = update_list[len(update_list) // 2].value
        middle_update_sum += middle_number

    return middle_update_sum

def update_list_valid(update_list, rules):
    for rule in rules:
        if not rule.validate_update_list(update_list):
            return False

    return True

if __name__ == "__main__":

    rules_pattern = re.compile(r"(\d+)\|(\d+)")
    updates_pattern = re.compile(r"\d+")

    parsed_rules = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            break
        match = rules_pattern.match(line)
        if match:
            num1, num2 = map(int, match.groups())
            parsed_rules.append(Rule(num1, num2))
        else:
            raise Exception("Invalid input")

    # print(parsed_rules)

    update_lists = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            break
        match = updates_pattern.findall(line)
        if match:
            update_lists.append(list(map(lambda val : PageUpdate(int(val), parsed_rules), match)))
        else:
            raise Exception("Invalid input")

    valid_update_lists = []
    invalid_update_lists = []
    for update_list in update_lists:
        if update_list_valid(update_list, parsed_rules):
            valid_update_lists.append(update_list)
        else:
            invalid_update_lists.append(update_list)

    middle_update_sum = add_middle_values(valid_update_lists)

    print(f"Part 1: {middle_update_sum}")


    reordered_update_lists = []
    for update_list in invalid_update_lists:
        reordered_update_list = sorted(update_list)
        reordered_update_lists.append(reordered_update_list)

    for i in range(0, len(reordered_update_lists)):
        print(f"{invalid_update_lists[i]} -> {reordered_update_lists[i]}: {update_list_valid(reordered_update_lists[i], parsed_rules)}")

    print(f"Part 2: {add_middle_values(reordered_update_lists)}")

    