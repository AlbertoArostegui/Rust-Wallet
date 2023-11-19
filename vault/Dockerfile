FROM hashicorp/vault:1.8.0

ENV VAULT_ADDR=http://127.0.0.1:8200

RUN apk add --no-cache jq

COPY config.json /vault/config/config.json
COPY unseal_vault.sh /unseal_vault.sh
COPY api.hcl /vault/config/api.hcl

RUN chmod +x /unseal_vault.sh && \
    chown vault:vault /unseal_vault.sh

CMD ["/unseal_vault.sh"]