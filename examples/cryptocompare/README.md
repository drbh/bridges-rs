## Cryptocompare

### Start Server

```bash
cargo run --example cryptocompare
```

### Call endpoint

```bash
curl --request POST \
  --url http://localhost:8081/ \
  --header 'Content-Type: application/json' \
  --data '{
	"jobRunId": "test"
}'
```
