""""Clase abstracta para la creacion de objetos scrapper"""
from enum import Enum
from amazon_scrapper import AmazonScrapper
from i_scrapper import IScrapper

class ScrapperType(Enum):
    """Enum class for scrapper types"""
    GOOGLE = 1
    BING = 2
    YAHOO = 3
    AMAZON = 4

class IScrapperFactory():
    """Creacion de objetos scrapper"""
    def __init__(self):
        self._switcher = {
            ScrapperType.AMAZON: AmazonScrapper()
        }

    def get_scrapper(self, scrapper_type) -> IScrapper:
        """Devuelve un objeto scrapper del tipo especificado"""
        return self._switcher.get(scrapper_type, None)
