#!/bin/bash

# 클라이언트 개인키 생성
openssl genrsa -out echo-client-key.pem 2048

# 클라이언트 CSR 생성
openssl req -new -key echo-client-key.pem -out echo-client.csr -subj "/CN=echo-client"

# 클라이언트 인증서 생성 (Root CA로 서명)
openssl x509 -req -in echo-client.csr -CA rootCA.pem -CAkey rootCA.key -CAcreateserial -out echo-client.pem -days 365 -sha256 -extfile <(cat <<EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
[req_distinguished_name]
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = clientAuth
EOF
)