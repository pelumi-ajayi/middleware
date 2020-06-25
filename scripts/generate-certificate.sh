#!/bin/bash
set -e

# cd $1

# prepare config file for root CA generation
cat <<EOF >> root.cnf
[ req ]
distinguished_name = req_dn
[ req_dn ]
[ v3_ca ]
basicConstraints = CA:TRUE
keyUsage = digitalSignature, nonRepudiation, keyCertSign, cRLSign
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always
EOF

ROOT_CA_DIR=.

ROOT_CA_KEY=$ROOT_CA_DIR/root-ca.key.pem
ROOT_CA=$ROOT_CA_DIR/root-ca.pem
ROOT_CA_DER=$ROOT_CA_DIR/root-ca.der

echo "Generate root CA key"
openssl genrsa -out $ROOT_CA_KEY 2048

echo "Generate root CA certificate"
openssl req -x509 -new -key $ROOT_CA_KEY -out $ROOT_CA -days 365 -SHA256 -subj "/C=NG/ST=Lagos/O=Middleware Services" -config root.cnf -extensions v3_ca
openssl x509 -outform der -in $ROOT_CA -out $ROOT_CA_DER

rm root.cnf

# prepare config file for server certificate generation
cat <<EOF >> server.cnf
extendedKeyUsage=serverAuth
subjectAltName = @alt_names
[alt_names]
DNS.1 = *.middleware.com
EOF

SERVER_CA_DIR=.
SERVER_KEY=$SERVER_CA_DIR/server.key.pem
SERVER_CERT=$SERVER_CA_DIR/cert.pem
SERVER_CERT_DER=$SERVER_CA_DIR/cert.der
IDENTITY=../rsc/identity.p12
PASSPHRASE=

echo "Generate server key"
openssl genrsa -out $SERVER_KEY 2048

echo "Generate server certificate"
openssl req -out server.csr -key $SERVER_KEY -new -days 365 -SHA256 -subj "/C=NG/ST=Lagos/O=Middleware Services/CN=*.middleware.com"
openssl x509 -req -days 365 -SHA256 -in server.csr -CA $ROOT_CA -CAkey $ROOT_CA_KEY -CAcreateserial -out $SERVER_CERT -extfile server.cnf
openssl x509 -outform der -in $SERVER_CERT -out $SERVER_CERT_DER

openssl pkcs12 -export -out $IDENTITY -inkey $SERVER_KEY -in $SERVER_CERT -passout pass:$PASSPHRASE

rm server.csr
rm server.cnf
