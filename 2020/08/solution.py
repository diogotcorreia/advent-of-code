
lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

instructions = [{
    "op": line.split(" ")[0],
    "arg": int(line.split(" ")[1])}
    for line in lines]


def runCode(instructions):
    i = 0
    maxI = len(instructions)
    executed = [False] * maxI
    acc = 0
    currentInstruction = instructions[i]
    while i < maxI and not executed[i]:
        executed[i] = True
        op = currentInstruction["op"]
        arg = currentInstruction["arg"]
        if op == "acc":
            acc += arg
            i += 1
        elif op == "jmp":
            i += arg
        elif op == "nop":
            i += 1
        if i < maxI:
            currentInstruction = instructions[i]
    return (acc, i < maxI)

##########################################
#                 PART 1                 #
##########################################


def part1(instructions):
    acc, earlyExit = runCode(instructions)
    return acc


print('Answer to part 1 is', part1(instructions))

##########################################
#                 PART 2                 #
##########################################


def part2(instructions):
    # Brute force
    for i in range(len(instructions)):
        instruction = instructions[i]
        op = instruction["op"]
        if op == "acc":
            continue
        newOp = "nop" if op == "jmp" else "jmp"

        newInstruction = {
            "op": newOp,
            "arg": instruction["arg"]
        }

        acc, earlyExit = runCode(
            instructions[:i] + [newInstruction] + instructions[i + 1:])
        if not earlyExit:
            return acc


print('Answer to part 2 is', part2(instructions))
