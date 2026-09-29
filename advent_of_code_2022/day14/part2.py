import math

def get_offsets(full_list: list[list[tuple[int,int]]]) -> dict:
    """Read input file and get the lowest and highest values for each part of the comma-separated pairs."""
    first_values = []
    second_values = []

    for line in full_list:
        for pair in line:
            first_values.append(pair[0])
            second_values.append(pair[1])

    return min(first_values), min(second_values), max(first_values), max(second_values)

def get_file_lines(input_file: str) -> list[list[tuple[int,int]]]:
    full_list = []
    with open(input_file, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            # Split by '->' to get individual coordinate pairs
            pairs = [p.strip() for p in line.split('->')]
            sublist = []
            for pair in pairs:
                parts = pair.split(',')
                sublist.append((int(parts[0]), int(parts[1])))
            full_list.append(sublist)
    return full_list

def construct_rock_wall(line: list[tuple[int,int]], grid: list[list[str]]):
    prev_x = -1
    prev_y = -1

    for point in line:
        grid[point[1]][point[0]] = '#'
        if prev_x == -1:
            prev_x = point[0]
            prev_y = point[1]
            continue

        for i in range(prev_x, point[0], int(math.copysign(1, point[0]-prev_x))):
            x = i
            y = prev_y
            grid[y][x] = '#'
        for i in range(prev_y, point[1], int(math.copysign(1, point[1]-prev_y))):
            x = prev_x
            y = i
            grid[y][x] = '#'

        prev_x = point[0]
        prev_y = point[1]

def drip_sand(coordinate: int, grid: list[list[str]]) -> bool:
    occupied_spaces = {'#', 'o'}
    if grid[0][coordinate] in occupied_spaces:
        return False
    row_count = 0
    x = coordinate
    for row_count, row in enumerate(grid):
        if row[x] in occupied_spaces:
            if row[x-1] in occupied_spaces:
                if row[x+1] in occupied_spaces:
                    if grid[row_count-1][x] not in occupied_spaces:
                        grid[row_count-1][x] = 'o'
                        return True
                    else:
                        return False
                else:
                    x = x + 1
            else: x = x - 1

    return False

def print_grid(grid: list[list]):
    grid_min_offset = len(grid[0])
    grid_max_offset = 0
    for i in range(len(grid)-1):
        for j in range(len(grid[i])):
            if grid[i][j] != '.':
                grid_min_offset = min(grid_min_offset, j)
                grid_max_offset = max(grid_max_offset, j)

    for line in grid:
        print(''.join(line[grid_min_offset:grid_max_offset+1]))

def main() -> None:
    data = get_file_lines('input.txt')
    _, _, x_max, y_max = get_offsets(data)
    grid = []
    for i in range(y_max+2):
        grid.append(['.']*(x_max+y_max+1))

    for line in data:
         construct_rock_wall(line, grid)

    grid.append(['#']*(x_max+y_max+1))

    print_grid(grid)

    sand_counter = 0
    while drip_sand(500, grid):
        sand_counter += 1

    print_grid(grid)
    print(sand_counter)





main()