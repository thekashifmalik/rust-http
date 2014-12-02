from BaseHTTPServer import HTTPServer, BaseHTTPRequestHandler


class RequestHandler(BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.0'

    def do_GET(self):
        print self.headers

        self.send_response(200)

        self.send_header('X-Custom-Header', 'lol')
        self.end_headers()

        self.wfile.write('hello')
        self.wfile.flush()

httpd = HTTPServer(
    ('localhost', 13000),
    RequestHandler,
)

sa = httpd.socket.getsockname()
print "Serving HTTP on", sa[0], "port", sa[1], "..."
httpd.serve_forever()
