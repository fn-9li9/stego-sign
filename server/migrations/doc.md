<!-- docker run -d --name stego-db-test -e POSTGRES_USER=sre -e POSTGRES_PASSWORD=sre -e POSTGRES_DB=stegosign -p 55432:5432 postgres:18 -->
aplicar schemas (orden importa: files primero porque app depende de él)
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/schema_files.sql`
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/schema_app.sql`

reset datos
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/reset_schema_files.sql`
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/reset_schema_app.sql`

eliminar schemas completos
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/delete_schema_app.sql`
`docker exec -i stego-db-test psql -U sre -d stegosign < migrations/delete_schema_files.sql`