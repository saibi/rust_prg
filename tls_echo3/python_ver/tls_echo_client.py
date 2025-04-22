# tls_echo_client.py
import ssl
import socket

HOST = '127.0.0.1'
PORT = 8443

context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH, cafile='rootCA.pem')
context.load_cert_chain(certfile='echo-client.pem', keyfile='echo-client-key.pem')

with socket.create_connection((HOST, PORT)) as sock:
    with context.wrap_socket(sock, server_hostname='server.edger.dev') as ssock:
        print(f"Connected to {HOST}:{PORT}")
        ssock.sendall(b'Hello, TLS Echo Server!')
        data = ssock.recv(1024)
        print(f"Received: {data.decode()}")

