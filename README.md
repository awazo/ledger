# run
`sudo docker compose up -d`
port: 2480

# dev
`cp ./docker-compose.yml.dev ./docker-compose.yml`
`sudo docker compose up -d`
`sudo docker compose exec app bash`
(in app container): `cargo run`

# dump database
`sudo docker compose up -d`
`sudo docker compose exec db bash`
(in db container): `pg_dump -U postgres -cC --column-inserts --if-exists ledger > ./dump.txt`

# restore database
`sudo docker compose up -d`
`sudo cp ./dump.txt ./db/data/.`
`sudo docker compose exec db bash`
(in db container): `psql -U postgres < /var/lib/postgres/data/dump.txt`

# remove database
`sudo docker compose down --volumes`
`sudo rm -r ./db/data`
`mkdir ./db/data`

