"""
# DOCS
## Bibliotecas
- Beautiful soup
- Scrapy: Un framework de alto rendimiento ideal para proyectos de scraping a gran escala. Ofrece un sistema completo para gestionar solicitudes, programar spiders personalizados y procesar los datos obtenidos
- Selenium: Perfecta para automatizar interacciones con navegadores web y extraer contenido dinámico generado por JavaScript
"""

import time
import requests
from bs4 import BeautifulSoup

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
        # Parsear el contenido HTML
        soup = BeautifulSoup(respuesta.content, 'html.parser')
        # Encontrar todos los elementos de producto
        productos = soup.find_all('div', {'class': 'a-section a-spacing-base'})
        lista_productos = []
        for producto in productos:
            # Extraer el título del producto
            titulo = producto.find('div', {'data-cy': 'title-recipe'})
            if titulo:
                titulo = producto.find('h2').span.string
            
            # Extraer el precio del producto
            # precio = producto.find('span', {'class': 'a-price a-text-price'})
            precio = producto.find('span', {'class': 'a-offscreen'})
            if precio:
                precio = precio.text.strip()
            
            # Añadir el producto a la lista
            if titulo and precio:
                lista_productos.append({'titulo': titulo, 'precio': precio})
        
        return lista_productos
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
        print(f"Título: {producto['titulo']}")
        print(f"Precio: {producto['precio']}")
        print("---")
