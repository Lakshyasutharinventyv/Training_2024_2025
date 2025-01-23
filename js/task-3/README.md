### Project Description
This project implements a basic session-based authentication system using JavaScript. It allows users to register, log in, and access a protected page (`home.html`). Sessions are time-bound and automatically expire after a specified duration.



### Features
- User Registration: Allows users to register with a username, email, and password. User credentials are stored in `localStorage`.
- User Login:** Authenticates users by validating their credentials. Creates a session upon successful login.
- Session Management: Sessions have a defined expiry time. Active sessions are monitored, and expired sessions automatically log out the user.
- Automatic Logout: Users are redirected to the login page if the session is invalid or expired.



### Project Structure
- `index.html`: Contains the registration and login forms.
- `home.html`: A protected page accessible only with a valid session.
- `script.js`: Implements the authentication logic, session management, and form handling.



### How to Use
1. Registration
   - Open `index.html`.
   - Fill in the username, email, and password fields.
   - Click the "Register" button.
   - A success message will confirm registration.

2. Login
   - On the same page, switch to the login form.
   - Enter the registered username and password.
   - Upon successful login:
     - A session is created and saved in `sessionStorage`.
     - The user is redirected to `home.html`.

3. Session Handling
   - Once on `home.html`, the session is monitored:
     - If valid, the user remains on the page.
     - If expired, the user is automatically logged out and redirected to `index.html`.

4. Logout
   - The session can also be manually ended by calling the `logoutUser()` function.



### Code Highlights
#### Session Expiry
- Each session is valid for `42 seconds`:
  ```javascript
  const expiry = 42 * 1000; // 42 seconds
  ```

#### Session Object
- Example of a session object:
  ```javascript
  const session = {
    user: "username",
    expiresIn: 42000, // 42 seconds
    createdAt: 1690000000000, // Timestamp
  };
  ```

#### Session Validation
- The session age is calculated, and if it exceeds the expiry time, the user is logged out:
  ```javascript
  const sessionAge = new Date().getTime() - session.createdAt;
  if (sessionAge > expiry) {
    logoutUser();
  }
  ```

### **Dependencies**
No external libraries or frameworks are required; this project uses vanilla JavaScript.

