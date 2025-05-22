#!/bin/bash 

mkdir -p certs
touch certs/index.txt
echo 1000 > certs/serial

openssl genrsa -out certs/ca.key.pem 4096
openssl req -x509 -new -key certs/ca.key.pem \
  -days 3650 -out certs/ca.cert.pem \
  -config certs/openssl.cnf



# server cert

openssl genrsa -out certs/server.key.pem 2048

openssl req -new -key certs/server.key.pem \
  -out certs/server.csr.pem \
  -subj "/CN=localhost"

openssl ca -batch -config certs/openssl.cnf \
  -extensions server_cert \
  -in certs/server.csr.pem \
  -out certs/server.cert.pem -days 365 \
  -cert certs/ca.cert.pem -keyfile certs/ca.key.pem


# client cert

openssl genrsa -out certs/client.key.pem 2048

openssl req -new -key certs/client.key.pem \
  -out certs/client.csr.pem \
  -subj "/CN=test-client"

openssl ca -batch -config certs/openssl.cnf \
  -extensions client_cert \
  -in certs/client.csr.pem \
  -out certs/client.cert.pem -days 365 \
  -cert certs/ca.cert.pem -keyfile certs/ca.key.pem
