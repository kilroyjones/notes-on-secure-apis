from http.server import HTTPServer, SimpleHTTPRequestHandler
import ssl


httpd = HTTPServer(('localhost', 8000), SimpleHTTPRequestHandler)

httpd.socket = ssl.wrap_socket (httpd.socket, 
    keyfile="./localhost-key.pem", 
    certfile='./localhost.pem', server_side=True)

httpd.serve_forever()