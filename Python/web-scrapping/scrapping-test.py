"""
# DOCS
- Beautiful soup
- Scrapy: Un framework de alto rendimiento ideal para proyectos de scraping a gran escala. Ofrece un sistema completo para gestionar solicitudes, programar spiders personalizados y procesar los datos obtenidos
- Selenium: Perfecta para automatizar interacciones con navegadores web y extraer contenido dinámico generado por JavaScript
"""
from searcher import Searcher
from i_scrapper_factory import IScrapperFactory, ScrapperType

def obtener_productos_amazon(url, query):
    """Obtener los productos de la página de Amazon"""
    searcher = Searcher()
    respuesta = searcher.get(url, query)

    if respuesta is None:
        print("No se ha podido acceder a la página")
        return []

    scrapper = IScrapperFactory().get_scrapper(ScrapperType.AMAZON)
    if scrapper is  None:
        print("Scrapper no encontrado")
        return []

    scrapper.scrap(respuesta)
    return scrapper.get_products()

# URL de búsqueda de auriculares en Amazon España
BASE_URL = "https://www.amazon.com/s"
SEARCH_QUERY = "laptop"

# Obtener los productos
productos = obtener_productos_amazon(BASE_URL, SEARCH_QUERY)

# Imprimir los productos obtenidos
if len(productos) == 0:
    print("No se han encontrado productos")
else:
    for producto in productos:
        print(producto)
