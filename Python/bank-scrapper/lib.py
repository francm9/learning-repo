"""
Funciones auxiliares para el proyecto
"""
import requests

def request_token():
    """
    Solicita un request token a la API de OpenBankProject
    """
    url = "https://api3.openbankproject.com/oauth/initiate"
    res = requests.post(url, timeout=5)
    return res.text
