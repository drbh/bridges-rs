## Gasstation

### Start Server

```bash
cargo run --example gasstation
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
