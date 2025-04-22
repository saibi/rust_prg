# tls_echo_server.py
import ssl
import socket

HOST = '127.0.0.1'
PORT = 8443

context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
context.load_cert_chain(certfile='server.pem', keyfile='server-key.pem')
context.load_verify_locations(cafile='rootCA.pem')  # 루트 CA 로 클라이언트 cert 검증
context.verify_mode = ssl.CERT_REQUIRED

with socket.socket(socket.AF_INET, socket.SOCK_STREAM, 0) as sock:
    sock.bind((HOST, PORT))
    sock.listen(5)
    print(f"Server listening on {HOST}:{PORT}")

    with context.wrap_socket(sock, server_side=True) as ssock:
        while True:
            conn, addr = ssock.accept()
            print(f"Connection from {addr}")
            with conn:
                while True:
                    data = conn.recv(1024)
                    if not data:
                        break
                    print(f"Received: {data.decode()}")
                    conn.sendall(data)

