import requests


url = 'http://127.0.0.1:8200/v1/secret/data/api/secretkeys/' + 'emaildeprueba3@ep.com'
headers = {'X-Vault-Token': 's.FtD2GES22f2hLPyHrmx6O6S7'}

response = requests.get(url, headers=headers)
print(response)
data = response.json()

print(data)

password = data['data']['privatekey']

print(password)