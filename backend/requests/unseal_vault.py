import requests

VAULT_ADDR = 'http://127.0.0.1:8200'  # Replace with your Vault server address
UNSEAL_KEYS = ['A8HEWdEmMUaA6c5Ytf7cqo4sENYW54cfCHhCj0fY1Gc=']  # Replace with your actual unseal keys

def unseal_vault():
    for key in UNSEAL_KEYS:
        data = {'key': key}
        response = requests.put(f'{VAULT_ADDR}/v1/sys/unseal', json=data)
        if response.status_code != 200:
            print(f"Error unsealing Vault with key {key}: {response.text}")
            break
        print(f"Vault unsealed with key {key}: {response.json()}")

if __name__ == "__main__":
    unseal_vault()
