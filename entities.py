class World:
    def __init__(self):
        self.cities = []

class City:
    def __init__(self):
        self.players = []
        self.shops = []
        self.prices = {}

    def check_products(self, product):
        shops_count = 0
        product_count = 0
        sales_sum = 0
        price_sum = 0

        for shop in self.shops:
            if product in shop.items.keys() and shop.items[product]['stock'] > 0:
                shops_count += 1
                product_count += shop.items[product]['stock']
                sales_sum += shop.items[product]['sales']
                price_sum += shop.items[product]['price']
        if shops_count == 0:
            return
        product_average = product_count / shops_count
        sales_average = sales_sum / shops_count
        price_average = price_sum / shops_count

        if product in self.prices:
            data = {
                'amount_avg': product_average,
                'sales_avg': sales_average,
                'price_avg': price_average
            }
            self.prices[product].append(data)
        else:
            self.prices[product] = []
            data = {
                'amount_avg': product_average,
                'sales_avg': sales_average,
                'price_avg': price_average
            }
            self.prices[product].append(data)
        
        inflation_rate = 0

        if len(self.prices[product]) > 1:
            inflation_rate = (self.prices[product][-1]['price_avg'] - self.prices[product][0]['price_avg']) / self.prices[product][0]['price_avg']
        

        print(f"Taxa de inflação de {product}: {inflation_rate}")
        print(f"Existem {shops_count} lojas vendendo {product}.")
        print(f"Media de estoque por loja: {product_average}")
        print(f"Media de vendas por loja: {sales_average}")
        print(f"Media de preco por loja: {price_average}")

