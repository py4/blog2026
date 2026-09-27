"""Serve the generated site locally, including its extensionless page links."""

from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit, urlunsplit


SITE = Path(__file__).resolve().parent / "dist"


class SiteHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(SITE), **kwargs)

    def resolve_page(self):
        parts = urlsplit(self.path)
        name = parts.path.lstrip("/")
        if name and not Path(name).suffix and (SITE / f"{name}.html").is_file():
            self.path = urlunsplit((parts.scheme, parts.netloc, f"/{name}.html", parts.query, parts.fragment))

    def do_GET(self):
        self.resolve_page()
        super().do_GET()

    def do_HEAD(self):
        self.resolve_page()
        super().do_HEAD()


if __name__ == "__main__":
    server = ThreadingHTTPServer(("127.0.0.1", 8000), SiteHandler)
    print("Preview at http://127.0.0.1:8000/", flush=True)
    server.serve_forever()
