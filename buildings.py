from beings import Food
from foods import food_generator, foods


class Shop:
    def __init__(self):
        # Cada item tem pre5ço base, estoque e histórico de vendas e compras
        self.items = {
            "Espada": {"price": 50, "stock": 10, "sales": 0, "purchases": 0},
            "Poção": {"price": 10, "stock": 20, "sales": 0, "purchases": 0},
            "Armadura": {"price": 100, "stock": 5, "sales": 0, "purchases": 0},
            "Potato": {"price": 5, "stock": 100, "sales": 0, "purchases": 0},
            "Corn": {"price": 1, "stock": 0, "sales": 0, "purchases": 0},
        }
        self.transaction_count = 0  # Contador global de transações

    def show_items(self):
        print("Loja - Itens disponíveis:")
        for item, data in self.items.items():
            price = self.calculate_price(data["price"], data["sales"], data["purchases"])
            print(f"{item}: {price} ouro (Estoque: {data['stock']})")

    def calculate_price(self, base_price, sales, purchases):
        """
        Aplica inflação/deflação a cada 10 transações.
        """
        if self.transaction_count % 10 != 0:  # Só aplica a cada 10 transações
            return base_price
        inflation_rate = sales * 0.05  # Cada venda aumenta 5% do preço
        deflation_rate = purchases * 0.03  # Cada compra reduz 3% do preço
        adjusted_price = base_price * (1 + inflation_rate - deflation_rate)
        return max(int(adjusted_price), 1)  # Preço mínimo de 1 ouro

    def update_base_prices(self):
        """
        Atualiza os preços base a cada 100 transações e reseta o histórico.
        """
        # print("Atualizando preços base com base no histórico...")
        for item, data in self.items.items():
            sales = data["sales"]
            purchases = data["purchases"]
            # Calcula o novo preço base com base no histórico
            inflation_rate = sales * 0.05
            deflation_rate = purchases * 0.03
            new_price = data["price"] * (1 + inflation_rate - deflation_rate)
            data["price"] = max(int(new_price), 1)  # Atualiza o preço base
            # Reseta o histórico de vendas e compras
            data["sales"] = 0
            data["purchases"] = 0
        # print("Preços base atualizados com sucesso!")

    def check_transaction_limits(self):
        """
        Verifica os limites de 10 e 100 transações para aplicar as regras.
        """
        if self.transaction_count % 10 == 0:
            self.update_base_prices()
            # print(f"Transação {self.transaction_count}: Preços ajustados com inflação/deflação.")

    def buy_item(self, player, item):
        """
        Jogador compra um item da loja.
        """
        if item not in self.items:
            print(f"{item} não está disponível na loja.")
            return
        item_data = self.items[item]
        price = item_data["price"]
        if item_data["stock"] <= 0:
            # print(f"{item} está esgotado.")
            return
        if player.action("spend", price):
            item = next(food_generator(item))
            player.add_item(item, 1)
            item_data["stock"] -= 1
            item_data["sales"] += 1  # Registro da venda
            self.transaction_count += 1  # Incrementa o contador de transações
            self.check_transaction_limits()
            # print(f"{player.name} comprou {item} por {price} ouro.")

    def sell_item(self, player, item):
        """
        Jogador vende um item para a loja.
        """
        if item not in player.inventory or player.inventory[item] <= 0:
            print(f"{player.name} não tem {item} para vender.")
            return
        item_name = item.name
        item_data = self.items[item_name]
        # sell_price = self.calculate_price(self.items[item_name]["price"], self.items[item_name]["sales"], self.items[item_name]["purchases"]) // 2
        sell_price = item_data["price"]
        player.inventory[item] -= 1
        if player.inventory[item] == 0:
            del player.inventory[item]
        player.action("earn", sell_price)
        self.items[item_name]["stock"] += 1
        self.items[item_name]["purchases"] += 1  # Registro da compra
        self.transaction_count += 1  # Incrementa o contador de transações
        self.check_transaction_limits()
        # print(f"{player.name} vendeu {item} por {sell_price} ouro.")