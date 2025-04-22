#!/bin/bash

# 서버 개인키 생성
openssl genrsa -out server-key.pem 2048

# 서버 CSR(Certificate Signing Request) 생성
openssl req -new -key server-key.pem -out server.csr -subj "/CN=*.edger.dev"

# 서버 인증서 생성 (Root CA로 서명)
openssl x509 -req -in server.csr -CA rootCA.pem -CAkey rootCA.key -CAcreateserial -out server.pem -days 365 -sha256 -extfile <(cat <<EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
[req_distinguished_name]
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names
[alt_names]
DNS.1 = *.edger.dev
IP.1 = 127.0.0.1
EOF
)