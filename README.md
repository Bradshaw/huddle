# Huddle

A simple virtual tabletop

## Running Huddle

- Create a `.env` file at the root of the repository

```.env
# Client config
CLIENT_EXPOSED_PORT=3000
CLIENT_INTERNAL_PORT=80

# Server config
SERVER_LOG_LEVEL=info
SERVER_PORT=3001
SERVER_HOSTNAME=server
SERVER_WATCH_PROFILE=dev
SERVER_WATCH_SUBDIR=debug

# Database config
DB_DBNAME=example
DB_HOST=db
DB_USER=postgres
DB_PASSWORD=mysecretpassword
```

### Running in production

- `docker-compose up`
- The webapp will be served at `http://localhost:3000`

### Running in development with live reloading

- `docker compose -f compose.yaml -f compose.dev.yaml watch`
- The webapp will be served at `http://localhost:3000`
- Changes to the client or the server will trigger a rebuild of the appropriate service