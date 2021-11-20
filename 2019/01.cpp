#include <iostream>

int main()
{
  long fuelRequired = 0;
  long additionalFuel = 0;

  while (std::cin)
  {
    long mass = 0;
    std::cin >> mass;
    if (!std::cin.fail())
    {
      // Part 1
      mass /= 3;
      mass -= 2;
      fuelRequired += mass;

      // Part 2
      while (mass > 0)
      {
        additionalFuel += mass;
        mass /= 3;
        mass -= 2;
      }
    }
  }

  std::cout << "Part 1: " << fuelRequired << std::endl;
  std::cout << "Part 2: " << additionalFuel << std::endl;
}
