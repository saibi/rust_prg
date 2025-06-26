#!/bin/bash
DIR=$(dirname "$0")
WORK_DIR=$DIR/certs
CONF_FILE=$WORK_DIR/openssl.cnf
ROOTCA_CERT=$WORK_DIR/ca.cert.pem
ROOTCA_KEY=$WORK_DIR/ca.key.pem

if [ ! -d "$WORK_DIR" ] || [ ! -f "$ROOTCA_CERT" ]; then
    echo "Root CA not found."
    exit 1
fi

if [ $# -lt 2 ]; then
    echo "Usage: $0 <server|client|both> <CN> [days=365]"
    exit 1
fi
TYPE=$1
if [ "$TYPE" != "server" ] && [ "$TYPE" != "client" ] && [ "$TYPE" != "both" ]; then
    echo "Invalid type: $TYPE. Use 'server', 'client', or 'both'."
    exit 1
fi
CN=$2

DAYS=365
if [ $# -ge 3 ]; then
    DAYS=$3
fi

KEY_FILE=$WORK_DIR/$CN.key.pem
CSR_FILE=$WORK_DIR/$CN.csr.pem
CERT_FILE=$WORK_DIR/$CN.cert.pem
KEY_BITS=2048

openssl genrsa -out $KEY_FILE $KEY_BITS

openssl req -new -key $KEY_FILE \
    -out $CSR_FILE \
    -subj "/CN=$CN"

if [ "$TYPE" == "server" ]; then
    EXT_CONF=$WORK_DIR/server_ext.cnf

    cat >$EXT_CONF <<EOF
[server_cert]
subjectAltName = @server_alt_names

[server_alt_names]
DNS.1 = localhost
DNS.2 = $CN
EOF

    openssl ca -batch -config $CONF_FILE \
        -extensions server_cert \
        -extfile $EXT_CONF \
        -in $CSR_FILE \
        -out $CERT_FILE -days $DAYS \
        -cert $ROOTCA_CERT -keyfile $ROOTCA_KEY
elif [ "$TYPE" == "both" ]; then
    EXT_CONF=$WORK_DIR/both_ext.cnf

    cat >$EXT_CONF <<EOF
[both_cert]
subjectAltName = @both_alt_names
extendedKeyUsage = serverAuth, clientAuth

[both_alt_names]
DNS.1 = localhost
DNS.2 = $CN
EOF

    openssl ca -batch -config $CONF_FILE \
        -extensions both_cert \
        -extfile $EXT_CONF \
        -in $CSR_FILE \
        -out $CERT_FILE -days $DAYS \
        -cert $ROOTCA_CERT -keyfile $ROOTCA_KEY
else
    openssl ca -batch -config $CONF_FILE \
        -extensions client_cert \
        -in $CSR_FILE \
        -out $CERT_FILE -days $DAYS \
        -cert $ROOTCA_CERT -keyfile $ROOTCA_KEY
fi

openssl x509 -in $CERT_FILE -text -noout

echo "Certificate generated:"
echo "  Key: $KEY_FILE"
echo "  CSR: $CSR_FILE"
echo "  Cert: $CERT_FILE"
echo "Validity: $DAYS days"
echo "You can now use this certificate for $TYPE authentication."
