import hvac

client = hvac.Client(
    url='http://127.0.0.1:8200',
    token='s.FuZ34f8MMqA35Hi0I4nLmFHr',
)


create_response = client.secrets.kv.v2.create_or_update_secret(
    path='api/peli',
    secret=dict(contra='contra123'),
)

print('Secret written successfully.')
