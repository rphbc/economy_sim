
class Food:
    def __init__(self, name, energy):
        self.name = name
        self.energy = energy
    def __str__(self):
        return self.name
    def __repr__(self):
        return self.name

potato = Food("Potato", 40)
corn = Food("Corn", 20)
meat = Food("Meat", 50)

foods = {
    'Potato': potato,
    'Corn': corn,
    'Meat': meat
}

def food_generator(name):
    energy = foods[name].energy
    yield Food(name, energy)