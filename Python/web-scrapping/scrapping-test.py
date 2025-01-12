"""
# DOCS
## Bibliotecas
- Beautiful soup
- Scrapy: Un framework de alto rendimiento ideal para proyectos de scraping a gran escala. Ofrece un sistema completo para gestionar solicitudes, programar spiders personalizados y procesar los datos obtenidos
- Selenium: Perfecta para automatizar interacciones con navegadores web y extraer contenido dinámico generado por JavaScript
"""

import requests
from amazon_scrapper import AmazonScrapper

def obtener_productos_amazon(url, params):
    # Cabeceras para simular un navegador real
    headers = {
        "User-Agent": "MMozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        "Accept-Language": "es-ES,es;q=0.9"
    }

    # Realizar la petición GET
    respuesta = requests.get(url, params=params, headers=headers)

    # Verificar si la petición fue exitosa
    if respuesta.status_code == 200:
        # Crear un objeto de la clase AmazonScrapper
        scrapper = AmazonScrapper()
        # Llamar al método scrap con el contenido HTML de la página
        scrapper.scrap(respuesta.text)
        # Devolver los productos obtenidos
        return scrapper.get_products()
    else:
        print(f"Error al acceder a la página: {respuesta.status_code}")
        return []

# URL de búsqueda de auriculares en Amazon España
base_url = "https://www.amazon.com/s"
search_query = "laptop"  # Cambia esto por tu término de búsqueda
params = {
    "k": search_query,
    "ref": "nb_sb_noss"
}

# Obtener los productos
productos = obtener_productos_amazon(base_url, params)

# Imprimir los productos obtenidos
if len(productos) == 0:
    print("No se han encontrado productos")
else:
    for producto in productos:
        print(producto)
