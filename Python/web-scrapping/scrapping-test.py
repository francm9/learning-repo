"""
# DOCS
- Beautiful soup
- Scrapy: Un framework de alto rendimiento ideal para proyectos de scraping a gran escala. Ofrece un sistema completo para gestionar solicitudes, programar spiders personalizados y procesar los datos obtenidos
- Selenium: Perfecta para automatizar interacciones con navegadores web y extraer contenido dinámico generado por JavaScript
"""
import requests
from i_scrapper_factory import IScrapperFactory, ScrapperType

def obtener_productos_amazon(url, params):
    # Cabeceras para simular un navegador real
    headers = {
        "User-Agent": "MMozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        "Accept-Language": "es-ES,es;q=0.9"
    }

    # Realizar la petición GET
    respuesta = requests.get(url, params=params, headers=headers, timeout=5)

    # Verificar si la petición fue exitosa
    if respuesta.status_code == 200:
        scrapper = IScrapperFactory().get_scrapper(ScrapperType.BING)
        if scrapper is  None:
            print("Scrapper no encontrado")
            return []

        scrapper.scrap(respuesta.text)
        return scrapper.get_products()

    print(f"Error al acceder a la página: {respuesta.status_code}")
    return []

# URL de búsqueda de auriculares en Amazon España
BASE_URL = "https://www.amazon.com/s"
SEARCH_QUERY = "laptop"  # Cambia esto por tu término de búsqueda
params = {
    "k": SEARCH_QUERY,
    "ref": "nb_sb_noss"
}

# Obtener los productos
productos = obtener_productos_amazon(BASE_URL, params)

# Imprimir los productos obtenidos
if len(productos) == 0:
    print("No se han encontrado productos")
else:
    for producto in productos:
        print(producto)
