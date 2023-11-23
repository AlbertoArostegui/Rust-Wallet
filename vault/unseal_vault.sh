#!/bin/sh
KEYS=/vault/file/keys.txt
if [ -f "$KEYS" ]; then

    echo "if"
    sleep 1
    vault server -config /vault/config/config.json > /dev/null 2>&1 &
    VAULT_PID=$!
    UNSEAL_KEY=$(grep 'Unseal Key 1:' /vault/file/keys.txt | awk '{print $NF}')
    vault operator unseal $UNSEAL_KEY
    wait $VAULT_PID

else
    echo "else"
    vault server -config /vault/config/config.json > /dev/null 2>&1 &

    VAULT_PID=$!

    while ! nc -z localhost 8200; do   
    sleep 0.1
    done


    vault operator init -key-shares=1 -key-threshold=1 > /vault/file/keys.txt 2>&1
    if [ $? -ne 0 ]; then
        echo "Failed to initialize Vault"
        cat /vault/file/keys.txt
        exit 1
    fi


    UNSEAL_KEY=$(grep 'Unseal Key 1:' /vault/file/keys.txt | awk '{print $NF}')
    ROOT_TOKEN=$(grep 'Initial Root Token:' /vault/file/keys.txt | awk '{print $NF}')

    vault operator unseal $UNSEAL_KEY

    vault login $ROOT_TOKEN

    vault secrets enable -path=secret kv-v2

    vault policy write api /vault/config/api.hcl
    vault token create -policy=api -format=json | jq -r '.auth.client_token' > /vault/config/api_token.txt

    wait $VAULT_PID
fi