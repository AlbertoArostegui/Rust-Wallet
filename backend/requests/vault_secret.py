import hvac

client = hvac.Client(
    url='http://127.0.0.1:8200',
    token='1234123412341234',
)

create_response = client.secrets.kv.v2.create_or_update_secret(
    path='mi-secreto',
    secret=dict(contra='contra123'),
)

