"""Definición de la clase encargada de realizar la gestión de las búsquedas"""
from requests import get

class Searcher():
    """Clase encargada de realizar las búsquedas en la web"""
    def __init__(self):
        self._headers = {
            "User-Agent": "MMozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
            "Accept-Language": "es-ES,es;q=0.9"
        }

    def get(self, url, query, timeout=5):
        """Realiza una petición GET a la URL proporcionada"""
        params = {
            "k": query,
            "ref": "nb_sb_noss"
        }

        response = get(url, params=params, headers=self._headers, timeout=timeout)
        if response.status_code != 200:
            print(f"Error al acceder a la página: {response.status_code}")
            return None

        return response.text
