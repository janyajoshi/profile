from http.server import SimpleHTTPRequestHandler, HTTPServer
from functools import partial
import os, json, urllib.request, logging

logging.basicConfig(
    level=logging.DEBUG,
    format='[%(asctime)s] "%(message)s"',
    datefmt='%d/%b/%Y %H:%M:%S'
)

def get_ip_location(ip: str) -> str:
    try:
        url = f"http://ip-api.com/json/{ip}"
        with urllib.request.urlopen(url, timeout=5) as response:
            data = response.read().decode("utf-8")
            return json.loads(data)
    except Exception:
        return ""

class CustomHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        # cwd = os.getcwd()
        # logging.info(f"[Request] CWD: {cwd}")

        host = self.headers.get("Host")
        full_url = f"{host}{self.path if self.path != '/' else ''}"
        user_agent = self.headers.get('User-Agent', '').lower()

        if self.path == "/rusted-graphs":
            self.send_response(302)
            self.send_header("Location", "https://rusted-graphs.janya.joshi-rj.in/")
            self.end_headers()
            return

        if self.path == '/static/commands.js':
            referer = self.headers.get("Referer")
            if not referer:
                self.send_error(403, "Forbidden: Invalid Referer")
                return
            
            referer = referer.lstrip("http://").lstrip("https://").rstrip("/")
            logging.info(f"Referer for commands: {referer}")
            commands_file = "commands-r" if referer.endswith("r") else "commands"
            file_path = os.path.join(self.directory, 'static', f'{commands_file}.js')
            logging.info(f"filepath: {file_path}")
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
            content = content.replace("{{FULL_URL}}", referer).replace("{{CURL_COMMAND_HERE}}", f"curl {referer}")
            self.send_response(200)
            self.send_header("Content-type", "text/javascript; charset=utf-8")
            self.send_header("Content-length", str(len(content.encode("utf-8"))))
            self.end_headers()
            self.wfile.write(content.encode("utf-8"))

        elif self.path not in ["/", "/r"]:
            return super().do_GET()
         
        else:
            logging.info("**** PAGE REQUEST ****")
            logging.info(f"[Request] IP: {self.client_address}")
            logging.info(f"[Request] Full URL: {full_url}")
            logging.info(f"[Request] User-Agent: {user_agent}")
            logging.info(f"[Request] IP Location: {get_ip_location(self.headers.get('X-Real-IP'))}")
            logging.info("************************")

            if "curl" in user_agent or "wget" in user_agent:
                filename = f"{'index-r' if full_url.endswith('/r') else 'index'}.txt"
                content_type = "text/plain; charset=utf-8"
            else:
                filename = "index.html"
                content_type = "text/html; charset=utf-8"

            file_path = os.path.join(self.directory, filename)
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
            content = content.replace("{{FULL_URL}}", full_url)
            self.send_response(200)
            self.send_header("Content-type", content_type)
            self.send_header("Content-length", str(len(content.encode("utf-8"))))
            self.end_headers()
            self.wfile.write(content.encode("utf-8"))

    def do_POST(self):
        if self.path == "/rg-metrics":
            content_length = int(self.headers.get("Content-Length", 0))
            body = self.rfile.read(content_length)
            raw_text = body.decode("utf-8", errors="ignore")

            with open(os.path.expanduser("~/rg-init-logs.txt"), "a", encoding="utf-8") as f:
                f.write(self.headers.get('X-Real-IP', "unknown") + ": " + raw_text + "\n")

            self.send_response(202)
            self.end_headers()

if __name__ == "__main__":
    PORT = 8000
    DIRECTORY = "_site"

    # Change working directory to _site
    handler_class = partial(CustomHandler, directory=DIRECTORY)

    with HTTPServer(("", PORT), handler_class) as httpd:
        logging.info(f"[Server] Serving _site at http://localhost:{PORT}")
        httpd.serve_forever()
