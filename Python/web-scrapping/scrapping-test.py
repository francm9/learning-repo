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

def obtener_productos_amazon(url):
    # Cabeceras para simular un navegador real
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
        'Accept-Language': 'es-ES,es;q=0.9,en;q=0.8'
    }
    
    # Realizar la petición GET
    respuesta = requests.get(url, headers=headers)
    
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
url = "https://www.amazon.es/s?k=ordenadore&__mk_es_ES=ÅMÅŽÕÑ&ref=nb_sb_noss"

# Obtener los productos
productos = obtener_productos_amazon(url)

# Imprimir los productos obtenidos
for producto in productos:
    print(f"Título: {producto['titulo']}")
    print(f"Precio: {producto['precio']}")
    print("---")

print("Fin de busqueda")

# Esperar un poco entre peticiones para no sobrecargar el servidor
time.sleep(2)
