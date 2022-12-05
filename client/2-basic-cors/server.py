from http.server import HTTPServer, SimpleHTTPRequestHandler
import ssl
import sys

port = 8000
if len(sys.argv) == 2:
    port = int(sys.argv[1])

httpd = HTTPServer(('localhost', port), SimpleHTTPRequestHandler)
print("Server running at http://localhost:" + str(port))
httpd.serve_forever()