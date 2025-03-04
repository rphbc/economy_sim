import random
import time
from beings import NPC
from buildings import Shop
from entities import City

# 1 Platinum = 100 GOLD
# 1 GOLD = 100 SILVER
# 1 SILVER = 100 COPPER

NUM_PLAYERS = 200
NUM_SHOPS = 10
NUM_CITIES = 10
START_GOLD = 20

def main():
    total_epoch = 0
    players = []
    player_average_health = 0
    player_average_hungry = 0
    shops = []
    stop = False
    shop = Shop()
    
    for i in range(NUM_SHOPS):
        shop = Shop()
        shops.append(shop)

    for i in range(NUM_PLAYERS):
        player = NPC(f"NPC {i+1}", start_money=START_GOLD)
        player.shops = shops
        players.append(player)

    city = City()
    city.shops = shops

    epoch = 0

    while not stop:
        count = 0
        player_average_health = 0
        player_average_hungry = 0
        player_average_gold = 0
        for player in players:
            if not player.alive:
                count += 1
                continue
            player.live()
            player.action("reason")
            player_average_health += player.health
            player_average_hungry += player.hungry
            player_average_gold += player.gold
            # if not player.alive:
            #     count += 1
        player_average_health /= len(players)
        player_average_hungry /= len(players)
        player_average_gold /= len(players)

        if count == len(players):
            stop = True
        
        if epoch % 10 == 0:
            city.check_products("Potato")
            city.check_products("Corn")
            print(f"Epoch: {total_epoch} - Num alive: {len(players) - count} PAGold: {player_average_gold} PAHealth: {player_average_health} PAHungry: {player_average_hungry}")
            epoch = 0
        epoch += 1
        total_epoch += 1
        time.sleep(0.2)

    # Jogador compra comida
    # shop.buy_item(player, "Food")

    # Jogador verifica o inventário
    # player.show_inventory()

    


if __name__ == '__main__':
    main()

