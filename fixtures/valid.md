# Create a pet

```json operation=createPet direction=request
{"name":"Ada","tag":"rescue"}
```

```curl operation=createPet direction=request
curl http://localhost:4010/pets \
  --header 'content-type: application/json' \
  --data '{"name":"Milo"}'
```
