#!/usr/bin/env python3
"""
Fusion Monolith Daemon (v0.1)

Implements docs/MONOLITH_PROTOCOL.md over HTTP+JSON.

POST /
Requests:
  {"action":"lookup","hash":"<sha256>","unit":"module"}
  {"action":"store","hash":"<sha256>","unit":"module","artifact":{...}}
  {"action":"diagnostics","project":"<id>"}

Storage:
  monolith/cache/<hash>.json
"""

from __future__ import annotations

import argparse
import json
import os
from http.server import BaseHTTPRequestHandler, HTTPServer

ROOT = os.path.dirname(__file__)
CACHE_DIR = os.path.join(ROOT, "cache")
os.makedirs(CACHE_DIR, exist_ok=True)

_DIAGS = {}  # project_id -> list of diagnostics

def _cache_path(h: str) -> str:
    return os.path.join(CACHE_DIR, f"{h}.json")

class Handler(BaseHTTPRequestHandler):
    def _send(self, code: int, obj: dict):
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(obj).encode("utf-8"))

    def do_POST(self):
        length = int(self.headers.get("content-length", 0))
        body = self.rfile.read(length).decode("utf-8") if length else "{}"
        try:
            req = json.loads(body)
        except Exception:
            return self._send(400, {"error": "invalid json"})

        action = req.get("action")

        if action == "lookup":
            h = req.get("hash")
            if not h:
                return self._send(400, {"error": "missing hash"})
            path = _cache_path(h)
            if os.path.exists(path):
                with open(path, "r", encoding="utf-8") as f:
                    artifact = json.load(f)
                return self._send(200, {"cached": True, "artifact": artifact, "timestamp": os.path.getmtime(path)})
            return self._send(200, {"cached": False})

        if action == "store":
            h = req.get("hash")
            artifact = req.get("artifact")
            if not h or artifact is None:
                return self._send(400, {"error": "missing hash or artifact"})
            path = _cache_path(h)
            with open(path, "w", encoding="utf-8") as f:
                json.dump(artifact, f, indent=2)
            return self._send(200, {"stored": True})

        if action == "diagnostics":
            project = req.get("project", "default")
            return self._send(200, {"items": _DIAGS.get(project, [])})

        if action == "push_diagnostic":
            project = req.get("project", "default")
            item = req.get("item")
            if not item:
                return self._send(400, {"error": "missing item"})
            _DIAGS.setdefault(project, []).append(item)
            return self._send(200, {"ok": True})

        return self._send(400, {"error": "unknown action"})

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", default=8085, type=int)
    args = ap.parse_args()

    srv = HTTPServer((args.host, args.port), Handler)
    print(f"Fusion Monolith v0.1 listening on http://{args.host}:{args.port} (cache={CACHE_DIR})")
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        print("shutting down")

if __name__ == "__main__":
    main()
