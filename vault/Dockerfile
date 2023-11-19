FROM hashicorp/vault:1.8.0

ENV VAULT_ADDR=http://127.0.0.1:8200

RUN apk add --no-cache jq

COPY config.json /vault/config/config.json
COPY vault-init.sh /vault-init.sh
COPY api.hcl /vault/config/api.hcl

RUN chmod +x /vault-init.sh && \
    chown vault:vault /vault-init.sh

CMD ["/vault-init.sh"]