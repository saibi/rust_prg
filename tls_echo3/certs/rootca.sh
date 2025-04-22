#!/bin/bash

# Root CA 개인키 생성
openssl genrsa -out rootCA.key 2048

# Root CA 인증서 생성 (20 years)
openssl req -x509 -new -nodes -key rootCA.key -sha256 -days 7300 -out rootCA.pem -subj "/CN=Edger Root CA"

