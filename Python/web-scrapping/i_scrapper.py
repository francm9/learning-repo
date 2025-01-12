""""Define los métodos necesarios para scrapear una página web"""
from abc import ABC, abstractmethod

class IScrapper(ABC):
    """"Define los métodos necesarios para scrapear una página web"""
    def __init__(self):
        self._products = []

    @abstractmethod
    def scrap(self, html):
        """Scrapea el html pasado por parámetro y devuelve una lista de productos"""    

    def get_products(self):
        """Devuelve la lista de productos obtenidos"""
        return self._products
