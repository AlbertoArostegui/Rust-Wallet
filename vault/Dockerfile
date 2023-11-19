FROM hashicorp/vault:1.8.0

ENV VAULT_ADDR=http://127.0.0.1:8200

COPY config.json /vault/config/config.json
COPY unseal_vault.sh /unseal_vault.sh

RUN chmod +x /unseal_vault.sh && \
    chown vault:vault /unseal_vault.sh

CMD ["/unseal_vault.sh"]