#!/usr/bin/env bash
set -e;

docker stop td-test-postgres || true && \
docker rm td-test-postgres || true && \
docker run -d --name td-test-postgres \
        -e POSTGRES_USER=postgres \
        -e POSTGRES_PASSWORD=postgres \
        -p 5432:5432 \
        postgres \
          -c ssl=on \
          -c ssl_cert_file=/etc/ssl/certs/ssl-cert-snakeoil.pem  \
          -c ssl_key_file=/etc/ssl/private/ssl-cert-snakeoil.key;
docker ps;


# TablePlus -> Right click connection -> Copy as url (params needed otherwise password is not used).
open "postgresql://postgres:postgres@127.0.0.1/postgres?statusColor=686B6F&enviroment=local&name=td-test-pg&tLSMode=0&usePrivateKey=false&safeModeLevel=0&advancedSafeModeLevel=0"


# To test via CLI:
# - PGPASSWORD=postgres psql "port=5432 host=127.0.0.1 user=postgres sslrootcert=/Users/enzo/Dev/rust-playground/unicon/sh/tls/ssl-cert-snakeoil.crt sslmode=verify-ca"
#       - Note:
#           - `verify-full` does not work.
#           - `ssl-cert-snakeoil.crt` is the server's cert (not the CA), but this still works.
#               - All all SSL certificates "self signed"?
#                   - https://en.wikipedia.org/wiki/Self-signed_certificate