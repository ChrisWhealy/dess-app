// Serves web/ on http://127.0.0.1:8080 with Node alone, for trying the build. A real deployment is any static host: see the README.
//
//   node scripts/serve.mjs [port]
import fs from 'node:fs'
import http from 'node:http'
import path from 'node:path'

const root = path.resolve(import.meta.dirname, '../web')
const port = Number(process.argv[2] ?? 8080)
const types = {
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.css': 'text/css',
  '.wasm': 'application/wasm',
  '.json': 'application/json'
}

http
  .createServer((request, response) => {
    const url = new URL(request.url, 'http://localhost')
    const file = path.join(
      root,
      path.normalize(decodeURIComponent(url.pathname === '/' ? '/index.html' : url.pathname))
    )
    if (!file.startsWith(root) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      response.writeHead(404).end('not found')
      return
    }
    response.writeHead(200, { 'content-type': types[path.extname(file)] ?? 'application/octet-stream' })
    fs.createReadStream(file).pipe(response)
  })
  .listen(port, '127.0.0.1', () => console.log(`serving ${root} on http://127.0.0.1:${port}`))
