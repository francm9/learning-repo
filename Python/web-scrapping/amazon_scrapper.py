"""
Esta clase se encarga de scrappear toda la información de una lista de productos de Amazon.
- Recibe un HTML de la página de Amazon y extrae la información de los productos.
"""
from bs4 import BeautifulSoup
from product import Product

class AmazonScrapper:
    """
    Esta clase se encarga de scrappear toda la información de una lista de productos de Amazon.
    - Recibe un HTML de la página de Amazon y extrae la información de los productos.
    """
    def __init__(self):
        self._products = []

    def parse_products(self, parsed_html):
        """Devuelve una lista con los productos no parseados"""
        return parsed_html.find_all('div', {'class': 'a-section a-spacing-base'})
    def get_product_name(self, product):
        """Devuelve el nombre del producto"""
        return product.find('div', {'data-cy': 'title-recipe'}).find('h2').span.string
    def get_price(self, product):
        """Devuelve el precio del producto"""
        return product.find('span', {'class': 'a-offscreen'}).text.strip()
    def get_description(self, product):
        """Devuelve la descripción del producto"""
        return product.find('span', {'class': 'a-offscreen'}).text.strip()
    def get_review(self, product):
        """Devuelve la valoración del producto"""
        return product.find('span', {'class': 'a-offscreen'}).text.strip()
    def scrap(self, html):
        """Scrapea el html pasado por parámetro y devuelve una lista de productos"""
        # Parseamos el contenido HTML
        parsed_content = BeautifulSoup(html, 'html.parser')

        # Obtenemos los productos no parseados
        non_parsed_products = self.parse_products(parsed_content)

        # Iteramos sobre los productos no parseados y creamos los objetos Product
        for product in non_parsed_products:
            product = Product(self.get_product_name(product),
                              self.get_price(product),
                              self.get_description(product),
                              self.get_review(product))
            if product.is_valid():
                self._products.append(product)
    def get_products(self):
        """Devuelve una lista con los productos"""
        return self._products
