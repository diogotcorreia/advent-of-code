import re
from functools import reduce

allIngredients = set()

ingredientsCount = {}


def countIngredients(ingredients):
    for i in ingredients:
        if i not in ingredientsCount:
            ingredientsCount[i] = 0
        ingredientsCount[i] += 1


def parseFood(line):
    global allIngredients
    ingredients, allergens = re.match(r"(.+) \(contains (.+)\)", line).groups()
    allIngredients.update(ingredients.split())
    countIngredients(ingredients.split())
    return tuple(ingredients.split()), tuple(allergens.split(', '))


foods = [parseFood(line.strip())
         for line in open("input.txt", 'r') if line.strip() != ""]


##########################################
#                 PART 1                 #
##########################################


def part1(foods):
    suspects = {}
    for ingre, allers in foods:
        for aller in allers:
            if aller in suspects:
                suspects[aller] = suspects[aller].intersection(set(ingre))
            else:
                suspects[aller] = set(ingre)

    allergenFree = allIngredients
    for aller in suspects:
        allergenFree = allergenFree.difference(suspects[aller])
    for aller in suspects:
        suspects[aller] = suspects[aller].difference(allergenFree)

    return reduce(lambda a, b: a + ingredientsCount[b], allergenFree, 0), suspects


allergenFreeCount, suspects = part1(foods)
print('Answer to part 1 is', allergenFreeCount)

##########################################
#                 PART 2                 #
##########################################


def part2(suspects):
    solution = {}
    while len(suspects.keys()) > 0:
        for aller in list(suspects):
            if len(suspects[aller]) == 1:
                ingredient = list(suspects[aller])[0]
                for aller2 in suspects:
                    suspects[aller2].discard(ingredient)
                solution[aller] = ingredient
                del suspects[aller]

    k = sorted(solution.keys())
    return ','.join([solution[allergen] for allergen in k])


print('Answer to part 2 is', part2(suspects))
