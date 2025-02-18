# Deck
A digital collection of pokemon cards.

## Migrations
```DATABASE_URL=$(kubectl get secret deck-db-app -n pokedb -o jsonpath='{.data.uri}' | base64 -d -) mirrord exec -- diesel migration run```
