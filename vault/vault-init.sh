#!/bin/sh

set -ex

unseal () {
    vault operator unseal $(grep 'Unseal Key 1:' /vault/config/keys.txt | awk '{print $NF}')
}

init () {
    vault server -config /vault/config/config.json > /dev/null 2>&1 &
    vault operator init -key-shares=1 -key-threshold=1 > /vault/config/keys.txt 2>&1
}

login () {
    vault login $(grep 'Initial Root Token:' /vault/config/keys.txt | awk '{print $NF}')
}

create_token () {
    vault token create -policy=api -format=json | jq -r '.auth.client_token' > /vault/config/api_token.txt
}

if [ -s /vault/config/keys.txt ]; then
    unseal
else
    init
    unseal
    login
    create_token
fi