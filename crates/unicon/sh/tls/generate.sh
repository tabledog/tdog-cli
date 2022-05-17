#!/bin/bash
# Generates public and private keys to be copied to a Postgres docker instance.
# - Not used as there are permisson errors when using macOS as a host.
# - Fix: Use Debian's `ssl-cert-snakeoil` certs.


# @see https://gist.github.com/mrw34/c97bb03ea1054afb551886ffc8b63c3b
# @see https://stackoverflow.com/a/55072885/4949386
openssl req -new -text -passout pass:abcd -subj /CN=localhost -out server.req -keyout privkey.pem;
openssl rsa -in privkey.pem -passin pass:abcd -out server.key;
openssl req -x509 -in server.req -text -key server.key -out server.crt;


# Issue: `could not load private key file` (due to docker mount mac os -> container) issue.
sudo chown 0:70 server.key;
sudo chmod 640 server.key;