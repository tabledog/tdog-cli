#!/usr/bin/env bash
set -e;
real=$(realpath "$(dirname "$0")");

# Note: `--platform x` need to work on Mac M1 (QEMU used for emulation).
# - MariaDB has native support for M1.
rm ./del/mysql-slow-query.log;
docker stop td-test-mysql || true && \
docker rm td-test-mysql || true && \
docker run --platform linux/amd64 --name td-test-mysql \
      -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw -d \
      -v $real/tls/dev-keys:/dev-keys \
      mysql:5.7 \
        --ssl \
        --ssl-cert=/dev-keys/ssl-cert-snakeoil.pem \
        --ssl-key=/dev-keys/ssl-cert-snakeoil.key;

docker ps;

# Run in background.
(sleep 15; $real/mysql-log-all-queries.sh) &


# TablePlus -> Right click connection -> Copy as url (params needed otherwise password is not used).
open "mysql://root:my-secret-pw@127.0.0.1?statusColor=686B6F&enviroment=local&name=test&tLSMode=0&usePrivateKey=false&safeModeLevel=0&advancedSafeModeLevel=0"