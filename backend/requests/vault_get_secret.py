import hvac

client = hvac.Client(
    url='http://127.0.0.1:8200',
    token='s.FuZ34f8MMqA35Hi0I4nLmFHr',
)

read_response = client.secrets.kv.read_secret_version(path='api/peli')

password = read_response['data']['data']['contra']

print(password)