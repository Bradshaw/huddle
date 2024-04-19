# Huddle

A simple virtual tabletop

## Running in production

- `docker-compose up`
- The webapp will be served at `http://localhost:3000`

## Running in development with live reloading

- `docker compose -f compose.yaml -f compose.dev.yaml watch`
- The webapp will be served at `http://localhost:3000`
- Changes to the client or the server will trigger a rebuild of the appropriate service