const http = require("http");
const querystring = require("querystring");

const server = http.createServer((req, res) => {
  if (req.url === "/login" && req.method === "GET") {
    res.writeHead(200, { "Content-Type": "text/html" });
    res.end(`
      <h1>Login Page</h1>
      <form id="loginForm" method="POST" action="/login">
        <input id="username" name="username" type="text" placeholder="Username" required>
        <input id="password" name="password" type="password" placeholder="Password" required>
        <button id="loginButton" type="submit">Login</button>
      </form>
    `);
  } else if (req.url === "/login" && req.method === "POST") {
    let body = '';
    

    req.on('data', chunk => {
      body += chunk;
    });

    req.on('end', () => {
      const { username, password } = querystring.parse(body); 
      console.log(`Received username: ${username}, password: ${password}`);

      if (username === 'test' && password === 'password') {
        console.log("Correct credentials");
        
        
        res.writeHead(302, { 'Location': '/dashboard' });
        res.end();
      } else {
        res.writeHead(401, { "Content-Type": "text/html" });
        res.end("<h1>Login Failed: Invalid credentials</h1>");
      }
    });
  } else if (req.url === "/dashboard") {
    res.writeHead(200, { "Content-Type": "text/html" });
    res.end("<h1>Welcome to the Dashboard</h1>");
  } else {
    res.writeHead(404, { "Content-Type": "text/plain" });
    res.end("404 Not Found");
  }
});

const PORT = 3000;
server.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}/login`);
});
