
import random
from foods import Food, food_generator



class Plant:
    def __init__(self, name):
        self.name = name
        self.has_fruit = False
        self.waiting_time = 0
        self.last_action = ""
    
    def reap(self):
        pass

    def grow(self):
        pass

    def live(self):
        pass
   
class PotatoTree(Plant):
    def __init__(self):
        super().__init__("Potato Tree")
        self.grow_time = 10
        self.fruit_quantity = 10
    
    def live(self):
        if self.last_action == "idle":
            self.has_fruit = True
        if self.waiting_time > 0:
            self.waiting_time -= 1
            # print(f"{self.name} esta {self.last_action} por {self.waiting_time} turnos.")
        if self.waiting_time <= 0:
            self.last_action = "idle"
        else:
            self.action("grow")
    
    def reap(self):
        self.has_fruit = False
        item = food_generator("Potato")
        return next(item), self.fruit_quantity
    
    def action(self, action, argument=None):
        if self.waiting_time > 0:
            # print(f"{self.name} ainda precisa esperar {self.waiting_time} turnos para realizar ação.")
            return
        if action == "grow":
            self.last_action = "growing"
            self.waiting_time = self.grow_time


class CornTree(Plant):
    def __init__(self):
        super().__init__("Corn Tree")
        self.grow_time = 6
        self.fruit_quantity = 20
    
    def live(self):
        if self.last_action == "idle":
            self.has_fruit = True
        if self.waiting_time > 0:
            self.waiting_time -= 1
            # print(f"{self.name} esta {self.last_action} por {self.waiting_time} turnos.")
        if self.waiting_time <= 0:
            self.last_action = "idle"
        else:
            self.action("grow")
    
    def reap(self):
        self.has_fruit = False
        item = food_generator("Corn")
        return next(item), self.fruit_quantity

    def action(self, action, argument=None):
        if self.waiting_time > 0:
            # print(f"{self.name} ainda precisa esperar {self.waiting_time} turnos para realizar ação.")
            return
        if action == "grow":
            self.last_action = "growing"
            self.waiting_time = self.grow_time


class Human:
    def __init__(self, name, vocation="undefined", location="city", start_money=100):
        self.name = name
        self.position = location
        self.shops = []
        self.alive = True
        self.health = 100
        self.hungry = 100
        self.vocation = vocation
        self.gold = start_money  # Moeda inicial
        self.inventory = {}  # Inventário vazio
        self.temp_inventory = {}
        self.waiting_time = 0
        self.last_action = "idle"
    
    def action(self, action, argument=None):
        if self.waiting_time > 0:
            # print(f"{self.name} ainda precisa esperar {self.waiting_time} turnos para realizar ação.")
            return
        if action == "eat":
            self.eat(argument, 2)
        elif action == "spend":
            return self.spend_gold(argument, 1)
        elif action == "earn":
            return self.earn_gold(argument, 1)
        elif action == "walk":
            self.walk(argument, 5)
        elif action == "seed":
            self.seed(1)
        elif action == "reap":
            self.reap(argument, 3)
        elif action == "check_grow":
            self.check_grow(argument)
        elif action == "reason":
            self.reason()
        else:
            print("Ação inválida.")

    def check_grow(self, turns=1):
        self.last_action = "checking growth"
        self.waiting_time = turns

    def eat(self, food: Food, turns=1):
        if food in self.inventory:
            self.last_action = "eating"
            self.waiting_time = turns
            self.hungry += food.energy
            if self.hungry > 100:
                self.hungry = 100
            # print(f"{self.name} comeu {food}. Sua fome aumentou para {self.hungry}.")
            self.inventory[food] -= 1
            if self.inventory[food] <= 0:
                del self.inventory[food]
        else:
            pass
            # print(f"{self.name} não tem {food} no inventário.")
    
    def walk(self,location, turns=5):
        self.last_action = "walking"
        self.waiting_time = turns
    
    def seed(self, turns=1):
        self.last_action = "seeding"
        self.waiting_time = turns
    
    def reap(self, plant: Plant, turns=1):
        self.last_action = "reaping"
        self.waiting_time = turns
        fruit, quantity = plant.reap()
        self.add_item(fruit, quantity, temp=True)
        for item in self.inventory:
                if isinstance(item, Plant):
                    del item     

    def reason(self):
        
        if self.hungry <= 50 and self.last_action == "idle": # human is hungry and idle
            for item in self.inventory:
                if isinstance(item, Food):
                    self.action("eat", item)
                    return
           
            if self.last_action == "idle" and self.position == "city":
                shop = random.choice(self.shops)
                choosen_food = random.choice(["Potato", "Corn"])
                shop.buy_item(self, choosen_food) # buy food
                return
            elif self.last_action == "idle" and self.position == "field":
                self.action("walk", "city")
                return
        
        if self.gold < 20 and self.last_action == "idle": # human is poor and idle
            for item in self.inventory:
                if isinstance(item, Food):
                    if self.position == "city":
                        shop = random.choice(self.shops)
                        shop.sell_item(self, item)
                        return
                    elif self.position == "field":
                        self.action("walk", "city")
                        return
            
            if self.last_action == "idle":
                if self.position == "city":
                    self.action("walk", "field")
                    return
                elif self.position == "field":
                    self.action("seed", "Potato")
                    return
        if self.last_action == "seeding":
            for item in self.inventory:
                if isinstance(item, Plant):
                    self.action("check_grow", 11)
                    return
            plant = random.choice([PotatoTree(), CornTree()])
            plant.action("grow")
            self.add_item(plant, 1)
            self.action("check_grow", 11)
            return
                    
            

    
    def live(self):
        if self.temp_inventory:
            for item in self.temp_inventory:
                if item not in self.inventory:
                    self.inventory[item] = self.temp_inventory[item]
                else:
                    self.inventory[item] += self.temp_inventory[item]
            self.temp_inventory = {}

        if self.health <= 0:
            self.alive = False
            print(f"{self.name} morreu.")
        
        self.hungry -= 1
        if self.hungry <= 0:
            self.health -= 1
            self.hungry = 0
        else:
            if self.health < 100:
                self.health += 1

        if self.waiting_time > 0:
            self.waiting_time -= 1
            # print(f"{self.name} esta {self.last_action} por {self.waiting_time} turnos.")
        else:
            if self.last_action == "walking" and self.position == "city":
                self.position = "field"
            elif self.last_action == "walking" and self.position == "field":
                self.position = "city"
            self.last_action = "idle"
        if self.last_action == "checking growth":
            for item in self.inventory:
                if isinstance(item, Plant):
                    if item.has_fruit:
                        self.action("reap", item)
                    else:
                        item.live()
                
        # print(f"{self.name} tem {self.health} de saude e {self.hungry} de fome. {self.name} esta {self.last_action}.")

    def earn_gold(self, amount, turns=1):
        self.last_action = "earning money"
        self.waiting_time = turns
        self.gold += amount
        # print(f"{self.name} ganhou {amount} de ouro. Saldo atual: {self.gold}")

    def spend_gold(self, amount, turns=1):
               
        if amount > self.gold:
            # print(f"{self.name} não tem ouro suficiente.")
            return False
        self.last_action = "spending money"
        self.waiting_time = turns
        self.gold -= amount
        # print(f"{self.name} gastou {amount} de ouro. Saldo atual: {self.gold}")
        return True

    def add_item(self, item, quantity, temp=False):
        if temp == True:
            if item in self.temp_inventory:
                self.temp_inventory[item] += quantity
            else:
                self.temp_inventory[item] = quantity
            # print(f"{self.name} recebeu {quantity}x {item}.")
        else:
            if item in self.inventory:
                self.inventory[item] += quantity
            else:
                self.inventory[item] = quantity
            # print(f"{self.name} recebeu {quantity}x {item}.")
    
    def show_inventory(self):
        print(f"Inventário de {self.name}: {self.inventory}")


class NPC(Human):
    def __init__(self, name, start_money=100):
        super().__init__(name=name, start_money=start_money)

