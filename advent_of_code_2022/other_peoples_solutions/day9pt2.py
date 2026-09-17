with open("Day9input.txt", "r") as file:
    instructions = [item.split(" ") for item in file.read().split("\n")]
head=[]
tail= []
positions = []
def part_one_setup():
    global head,tail, positions
    head = [0,0]
    tail = [0,0]
    positions = []
    for item in instructions:
        for command in range(0,int(item[1])):
            move(item[0])
            tail_copy = tail.copy()
            positions.append(tail_copy)
    return print(f"Part one: {len(set(tuple(item) for item in positions))}")

def part_two_setup():
    global head,tail,positions
    head = [0,0]
    tails = [[0,0] for item in range(0,9)]
    positions = []
    should_debug=False
    for num,item in enumerate(instructions):
        for command in range(0,int(item[1])):
            move_part_two(tails,item[0],should_debug)
            tail_copy = tails[-1].copy()
            positions.append(tail_copy)
    return print(f"Part two: {len(set(tuple(item) for item in positions))}")
    
def move(direction):
    if direction == "U":
        head[1] += 1
    elif direction =="D":
        head[1] -=1
    elif direction =="R":
        head[0] += 1
    else:
        head[0] -= 1
    positive_rc_conditions = [head[0]-tail[0] > 1, head[1] - tail[1] > 1] 
    negative_rc_conditions = [head[0]-tail[0] < -1, head[1] - tail[1] < -1] 
    positive_diag_conditions = [head[0] - tail[0] ]

    if head[0] == tail[0] or head[1] == tail[1]:
        if any(positive_rc_conditions):
            tail[positive_rc_conditions.index(True)] += 1
        elif any(negative_rc_conditions):
            tail[negative_rc_conditions.index(True)] -= 1
    else:
        if any(positive_rc_conditions):
            tail[positive_rc_conditions.index(True)] +=1
            if head[positive_rc_conditions.index(False)] - tail[positive_rc_conditions.index(False)] == 1:
                tail[positive_rc_conditions.index(False)] += 1
            else:
                tail[positive_rc_conditions.index(False)] -= 1
        if any(negative_rc_conditions):
            tail[negative_rc_conditions.index(True)] -= 1
            if head[negative_rc_conditions.index(False)] - tail[negative_rc_conditions.index(False)] == -1:
                tail[negative_rc_conditions.index(False)] -= 1
            else:
                tail[negative_rc_conditions.index(False)] += 1

def move_part_two(tails,direction,should_debug):
    if should_debug:
        pdb.set_trace()
    if direction == "U":
        head[1] += 1
    elif direction =="D":
        head[1] -=1
    elif direction =="R":
        head[0] += 1
    else:
        head[0] -= 1
    for count,tail in enumerate(tails):
        if count == 0:
            positive_rc_conditions = [head[0]-tail[0] > 1, head[1] - tail[1] > 1] #positive
            negative_rc_conditions = [head[0]-tail[0] < -1, head[1] - tail[1] < -1]   #
            if head[0] == tail[0] or head[1] == tail[1]:
                if any(positive_rc_conditions):
                    tail[positive_rc_conditions.index(True)] += 1
                elif any(negative_rc_conditions):
                    tail[negative_rc_conditions.index(True)] -= 1
            else:
                if all(positive_rc_conditions):
                    tail[0] += 1
                    tail[1] += 1
                elif any(positive_rc_conditions):
                    tail[positive_rc_conditions.index(True)] +=1
                    if head[positive_rc_conditions.index(False)] - tail[positive_rc_conditions.index(False)] == 1:
                        tail[positive_rc_conditions.index(False)] += 1
                    else:
                        tail[positive_rc_conditions.index(False)] -= 1
                elif all(negative_rc_conditions): #here
                    tail[0] -= 1
                    tail[1] -= 1
                elif any(negative_rc_conditions):
                    tail[negative_rc_conditions.index(True)] -= 1
                    if head[negative_rc_conditions.index(False)] - tail[negative_rc_conditions.index(False)] == -1:
                        tail[negative_rc_conditions.index(False)] -= 1
                    else:
                        tail[negative_rc_conditions.index(False)] += 1
        else:
            positive_rc_conditions = [tails[count-1][0]-tail[0] > 1, tails[count-1][1] - tail[1] > 1] 
            negative_rc_conditions = [tails[count-1][0]-tail[0] < -1, tails[count-1][1] - tail[1] < -1]   
            if tails[count-1][0] == tail[0] or tails[count-1][1] == tail[1]:
                if any(positive_rc_conditions):
                    tail[positive_rc_conditions.index(True)] += 1
                elif any(negative_rc_conditions):
                    tail[negative_rc_conditions.index(True)] -= 1
            else:
                if all(positive_rc_conditions):
                    tail[0] += 1
                    tail[1] += 1
                elif any(positive_rc_conditions):
                    tail[positive_rc_conditions.index(True)] +=1
                    if tails[count-1][positive_rc_conditions.index(False)] - tail[positive_rc_conditions.index(False)] == 1:
                        tail[positive_rc_conditions.index(False)] += 1
                    else:
                        tail[positive_rc_conditions.index(False)] -= 1
                elif all(negative_rc_conditions): #here
                    tail[0] -= 1
                    tail[1] -= 1
                elif any(negative_rc_conditions):
                    tail[negative_rc_conditions.index(True)] -= 1
                    if tails[count-1][negative_rc_conditions.index(False)] - tail[negative_rc_conditions.index(False)] == -1:
                        tail[negative_rc_conditions.index(False)] -= 1
                    else:
                        tail[negative_rc_conditions.index(False)] += 1

part_one_setup()
part_two_setup()

