from requests import get

base_url = "https://www.amazon.com/s"
search_query = "laptop"  # Cambia esto por tu término de búsqueda
params = {
    "k": search_query,
    "ref": "nb_sb_noss"
}

headers = {
    "User-Agent": "MMozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    "Accept-Language": "es-ES,es;q=0.9"
}
response = get(base_url, params=params, headers=headers)
print(response.text)
