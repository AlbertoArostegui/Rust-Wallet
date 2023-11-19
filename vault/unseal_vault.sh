#!/bin/sh

touch 1
vault server -config /vault/config/config.json > /dev/null 2>&1 &
touch 2

while ! nc -z localhost 8200; do   
  sleep 0.1
done
touch 3

vault operator init -key-shares=1 -key-threshold=1 > /vault/config/keys.txt 2>&1
if [ $? -ne 0 ]; then
  echo "Failed to initialize Vault"
  cat /vault/config/keys.txt
  exit 1
fi

UNSEAL_KEY=$(grep 'Unseal Key 1:' /vault/config/keys.txt | awk '{print $NF}')

vault operator unseal $UNSEAL_KEY