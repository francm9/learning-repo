"""
Este proyecto pretende scrappear la cuenta del banco para obtener los movimientos de la cuenta y poder tener
un control automatico de tus finanzas.

# DOCUMENTACION
- API3:  https://api3-explorer.openbankproject.com/?tags=&locale=en_GB
         https://api3-explorer.openbankproject.com/glossary#OAuth-1.0a
- PLAID: https://plaid.com/docs/api/products/balance/
- OpenBankProject: https://apiexplorer-ii-sandbox.openbankproject.com/glossary#Sandbox%20Introduction

# TAREA
- Conectar con la API de OpenBankProject
"""

# Importamos las librerias necesarias
from lib import request_token

# Definimos las variables necesarias
URL = "https://api3.openbankproject.com/oauth/initiate"

# Se solicita un request token
res = request_token()
print(res.text)
