const http = require('http');

var last_body = "";

var server = http.createServer(function (req, res) {
  if (req.method === "POST") {
    last_body = "";
    req.on('data', (buf) => last_body += buf.toString('utf8'));
    req.on('end', () => {
      res.writeHead(200);
      res.end();
    });
    req.resume();
  } else {
    res.writeHead(200);
    res.end(last_body);
  }
});

server.listen(8080);
