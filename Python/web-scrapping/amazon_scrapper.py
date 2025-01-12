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
        titulo = product.find('div', {'data-cy': 'title-recipe'})
        if titulo:
            return titulo.find('h2').span.string
        return "No encontrado"

    def get_price(self, product):
        """Devuelve el precio del producto"""
        precio = product.find('span', {'class': 'a-offscreen'})
        if precio:
            return precio.text.strip()
        return 0.0

    def get_description(self, product):
        """Devuelve la descripción del producto"""
        return "Descripcion no encontrada"

    def get_review(self, product):
        """Devuelve la valoración del producto"""
        review = product.find('i', {'data-cy': 'reviews-ratings-slot'})
        if review:
            return float(review.find('span').string.split()[0])
        return 0.0

    def get_products(self):
        """Devuelve una lista con los productos"""
        return self._products

    def scrap(self, html):
        """Scrapea el html pasado por parámetro y devuelve una lista de productos"""
        # Parseamos el contenido HTML
        parsed_content = BeautifulSoup(html, 'html.parser')

        # Obtenemos los productos no parseados
        non_parsed_products = self.parse_products(parsed_content)

        # Iteramos sobre los productos no parseados y creamos los objetos Product
        for product in non_parsed_products:
            parsed_product = Product(self.get_product_name(product),
                              self.get_price(product),
                              self.get_description(product),
                              self.get_review(product))

            if parsed_product.is_valid():
                self._products.append(parsed_product)
