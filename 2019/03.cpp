#include <iostream>
#include <unordered_map>
#include <string>

/* Kind of an hacky solution, not very efficient */
/* VERY cursed code, for your eye's safety, do not look */

using namespace std;

void calculate_next_position(int curr_x, int curr_y, char direction, int amount, int *next_x, int *next_y);
void fill_set(int curr_x, int curr_y, char direction, int amount, unordered_map<string, int> *set, int *step);
void check_collisions(int curr_x, int curr_y, char direction, int amount, unordered_map<string, int> *set, int *closest_x, int *closest_y, int *steps_to_collision, int *step);

int main()
{
  unordered_map<string, int> first_wire_path;

  // Run through first wire
  string wire1_input;
  cin >> wire1_input;

  int x = 0, y = 0, i = 0;
  char direction;
  int amount = 0;
  int steps = 0;

  for (auto &ch : wire1_input)
  {
    if (ch == ',')
    {
      int next_x, next_y;
      calculate_next_position(x, y, direction, amount, &next_x, &next_y);
      fill_set(x, y, direction, amount, &first_wire_path, &steps);
      x = next_x, y = next_y;
      direction = '\0';
      amount = 0;
    }
    else if (ch >= '0' && ch <= '9')
    {
      amount = amount * 10 + (ch - '0');
    }
    else
    {
      direction = ch;
    }
  }
  fill_set(x, y, direction, amount, &first_wire_path, &steps);

  string wire2_input;
  cin >> wire2_input;

  x = 0, y = 0, amount = 0, direction = '\0', steps = 0;
  int closest_x = 0, closest_y = 0;
  int steps_to_collision = 0;

  for (auto &ch : wire2_input)
  {
    if (ch == ',')
    {
      int next_x, next_y;
      calculate_next_position(x, y, direction, amount, &next_x, &next_y);
      check_collisions(x, y, direction, amount, &first_wire_path, &closest_x, &closest_y, &steps_to_collision, &steps);
      x = next_x, y = next_y;
      direction = '\0';
      amount = 0;
    }
    else if (ch >= '0' && ch <= '9')
    {
      amount = amount * 10 + (ch - '0');
    }
    else
    {
      direction = ch;
    }
  }
  check_collisions(x, y, direction, amount, &first_wire_path, &closest_x, &closest_y, &steps_to_collision, &steps);

  cout << "Part 1: " << abs(closest_x) + abs(closest_y) << endl;
  cout << "Part 2: " << steps_to_collision << endl;
}

void calculate_next_position(int curr_x, int curr_y, char direction, int amount, int *next_x, int *next_y)
{
  *next_x = curr_x;
  *next_y = curr_y;
  if (direction == 'U')
    *next_y += amount;
  else if (direction == 'D')
    *next_y -= amount;
  else if (direction == 'L')
    *next_x -= amount;
  else if (direction == 'R')
    *next_x += amount;
}

void fill_set(int curr_x, int curr_y, char direction, int amount, unordered_map<string, int> *set, int *step)
{
  while (amount-- > 0)
  {
    calculate_next_position(curr_x, curr_y, direction, 1, &curr_x, &curr_y);

    string pos_encoded = to_string(curr_x) + "," + to_string(curr_y);
    ++(*step);
    // only insert if not there, to not replace steps
    if (set->find(pos_encoded) == set->end())
      set->insert({pos_encoded, *step});
  }
}

void check_collisions(int curr_x, int curr_y, char direction, int amount, unordered_map<string, int> *set, int *closest_x, int *closest_y, int *steps_to_collision, int *step)
{
  while (amount-- > 0)
  {
    calculate_next_position(curr_x, curr_y, direction, 1, &curr_x, &curr_y);
    ++(*step);

    if (set->find(to_string(curr_x) + "," + to_string(curr_y)) != set->end())
    {
      if ((*closest_x == 0 && *closest_y == 0) || (abs(curr_x) + abs(curr_y) < abs(*closest_x) + abs(*closest_y)))
      {
        *closest_x = curr_x;
        *closest_y = curr_y;
      }
      int steps = (*set)[to_string(curr_x) + "," + to_string(curr_y)] + *step;
      if (*steps_to_collision == 0 || steps < *steps_to_collision)
      {
        cout << steps << ":" << *step << endl;
        *steps_to_collision = steps;
      }
    }
  }
}
