#include <iostream>
#include <string>

#define PROGRAM_SIZE 300
#define PART_TWO_OUTPUT_TARGET 19690720

void read_intcode_stdin(int *program);

void execute_program(int *program);

void duplicate_program(int *origin, int *target, int size);

int main()
{
  int program[PROGRAM_SIZE] = {0};
  read_intcode_stdin(program);

  // Part 1
  int part_one_program[PROGRAM_SIZE];
  duplicate_program(program, part_one_program, PROGRAM_SIZE);
  part_one_program[1] = 12;
  part_one_program[2] = 2;
  execute_program(part_one_program);

  // Part 2
  int part_two_program[PROGRAM_SIZE], noun, verb;
  for (noun = 0; noun <= 99 && part_two_program[0] != PART_TWO_OUTPUT_TARGET; ++noun)
  {
    for (verb = 0; verb <= 99 && part_two_program[0] != PART_TWO_OUTPUT_TARGET; ++verb)
    {
      duplicate_program(program, part_two_program, PROGRAM_SIZE);
      part_two_program[1] = noun;
      part_two_program[2] = verb;

      execute_program(part_two_program);
    }
  }

  std::cout << "Part 1: " << part_one_program[0] << std::endl;
  std::cout << "Part 2: " << 100 * (noun - 1) + (verb - 1) << std::endl;
}

void execute_program(int *program)
{
  int location = 0;
  int opcode;

  do
  {
    opcode = program[location++];
    if (opcode == 1)
    {
      // Add
      int locA = program[location++];
      int locB = program[location++];
      int locResult = program[location++];

      program[locResult] = program[locA] + program[locB];
    }
    else if (opcode == 2)
    {
      // Multiply
      int locA = program[location++];
      int locB = program[location++];
      int locResult = program[location++];

      program[locResult] = program[locA] * program[locB];
    }
    else if (opcode != 99)
    {
      exit(1); // something went wrong
    }
  } while (opcode != 99);
}

void duplicate_program(int *origin, int *target, int size)
{
  for (int i = 0; i < size; ++i)
  {
    target[i] = origin[i];
  }
}

void read_intcode_stdin(int *program)
{
  char c;
  int i = 0;
  int value = 0;

  while ((c = getchar()) != EOF)
  {
    if (c == ',')
    {
      program[i++] = value;
      value = 0;
    }
    else if (c >= '0' && c <= '9')
    {
      value *= 10;
      value += c - '0';
    }
  }
  program[i++] = value;
}
