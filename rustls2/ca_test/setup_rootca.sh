#!/bin/bash

DIR=$(dirname "$0")
WORK_DIR=$DIR/certs
CA_KEY=ca.key.pem
CA_CERT=ca.cert.pem
KEY_FILE=$WORK_DIR/$CA_KEY
CERT_FILE=$WORK_DIR/$CA_CERT
CONF_FILE=$WORK_DIR/openssl.cnf

DAYS=10950 # 30 years
KEY_SIZE=4096

if [ -d "$WORK_DIR" ] && [ -f "$CERT_FILE" ]; then
    echo "Root CA already exists. Skipping setup."
    exit 1
fi

mkdir -p "$WORK_DIR"
touch $WORK_DIR/index.txt
echo 1000 >$WORK_DIR/serial

# generate openssl.cnf
cat >$CONF_FILE <<EOF
[ ca ]
default_ca = CA_default

[ CA_default ]
dir               = ./certs
certs             = \$dir
new_certs_dir     = \$dir
database          = \$dir/index.txt
serial            = \$dir/serial
RANDFILE          = \$dir/.rand

private_key       = \$dir/$CA_KEY
certificate       = \$dir/$CA_CERT

default_md        = sha256
policy            = policy_loose
email_in_dn       = no
name_opt          = ca_default
cert_opt          = ca_default
copy_extensions   = copy

[ policy_loose ]
countryName             = optional
stateOrProvinceName     = optional
localityName            = optional
organizationName        = optional
organizationalUnitName  = optional
commonName              = supplied

[ req ]
default_bits       = 2048
prompt             = no
default_md         = sha256
distinguished_name = req_distinguished_name
x509_extensions    = v3_ca

[ req_distinguished_name ]
C  = KR
ST = Seoul
O  = saibi
CN = saibi CA

[ v3_ca ]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
basicConstraints = critical, CA:true
keyUsage = critical, keyCertSign, cRLSign

[ server_cert ]
basicConstraints = CA:FALSE
nsCertType = server
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth

[ client_cert ]
basicConstraints = CA:FALSE
nsCertType = client
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = clientAuth
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = localhost
EOF

# generate root CA
openssl genrsa -out $KEY_FILE $KEY_SIZE
openssl req -x509 -new -key $KEY_FILE \
    -days $DAYS -out $CERT_FILE \
    -config $CONF_FILE

echo "Root CA generated:"
echo "  Key: $KEY_FILE"
echo "  Cert: $CERT_FILE"
echo "Validity: $DAYS days"
echo "You can now use this CA to sign server and client certificates."
