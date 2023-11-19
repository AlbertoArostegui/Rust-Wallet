import hvac

client = hvac.Client(
    url='http://127.0.0.1:8200',
    token='s.s01qExUUMJYOOk2z362x07ad',
)

create_response = client.secrets.kv.v2.create_or_update_secret(
    path='api/peli',
    secret=dict(contra='contra123'),
)

