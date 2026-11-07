# A tiny local test server: /ok, /slow?ms=N, /status/N
import time
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

class H(BaseHTTPRequestHandler):
    def do_GET(self):
        u = urlparse(self.path)
        if u.path == "/ok":
            code = 200
        elif u.path == "/slow":
            time.sleep(int(parse_qs(u.query).get("ms", ["500"])[0]) / 1000)
            code = 200
        elif u.path.startswith("/status/"):
            code = int(u.path.split("/")[-1])
        else:
            code = 404
        self.send_response(code)
        self.end_headers()
        self.wfile.write(b"hello")
    def log_message(self, *a):
        pass

class Server(ThreadingHTTPServer):
    request_queue_size = 128

Server(("127.0.0.1", 8090), H).serve_forever()
