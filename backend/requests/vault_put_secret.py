import requests

url = 'http://127.0.0.1:8200/v1/secret/api/secretkeys/' + 'email26@gmail.com'
headers = {
    'X-Vault-Token': 's.JSEhbAvtxbPXF7da7iAFCSnf',
    'Content-Type': 'application/json'
}
data = {
        'privatekey': 'privarftek3432'
}


response = requests.post(url, headers=headers, json=data)
create_response = response.json()

if response.status_code == 200:
    print('Secret written successfully.')
else:
    print('Failed to write secret:', create_response['errors'])

