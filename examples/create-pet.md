# Create a pet

A current JSON request:

```json operation=createPet direction=request
{"name":"Miso","status":"available"}
```

A copied curl request with one retired field:

```curl operation=createPet direction=request
curl https://api.example.test/pets --data='{"name":"Pip","status":"adopted","retired_field":true}'
```
