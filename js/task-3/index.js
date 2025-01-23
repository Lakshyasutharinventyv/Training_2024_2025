const expiry = 42 * 1000; 

function setSession(username) {
  const session = {
    user: username,
    expiresIn:expiry,
    createdAt: new Date().getTime(),
  };
  sessionStorage.setItem('session', JSON.stringify(session));
}


function logoutUser() {
  sessionStorage.removeItem('session');
  window.location.assign("index.html"); 
}

function checkSessionValidity() {
  const session = JSON.parse(sessionStorage.getItem('session'));
  if (session) {
    const sessionAge = new Date().getTime() - session.createdAt;

    if (sessionAge > expiry) {
      logoutUser(); 
    }
  } else {
    logoutUser();
  }
}


document.addEventListener('DOMContentLoaded', () => {

  const registerForm = document.getElementById("registerForm");
  if (registerForm) {
    registerForm.addEventListener("submit", (e) => {
      e.preventDefault();
      const username = document.getElementById("registerUsername")?.value;
      const email = document.getElementById("registerEmail")?.value;
      const password = document.getElementById("registerPassword")?.value;

      if (!username || !email || !password) {
        alert("Please enter all credentials!");
        return;
      }

      localStorage.setItem(`${username}`, password);
      alert("Registration successful!");
    });
  }

  // Login form
  const loginForm = document.getElementById("loginForm");
  if (loginForm) {
    loginForm.addEventListener("submit", (e) => {
      e.preventDefault();
      const username = document.getElementById("loginUsername")?.value;
      const password = document.getElementById("loginPassword")?.value;

      if (!username || !password) {
        alert("Please enter all credentials!");
        return;
      }

      const originalPass = localStorage.getItem(`${username}`);
      if (!originalPass) {
        alert("No user found!");
        return;
      }

      if (originalPass === password) {
        setSession(username);
        localStorage.setItem(username,originalPass)
        window.location.assign("home.html");
    } else {
        alert("Invalid credentials!");
    }
    });
  }

});




window.onload = function(){
  if(window.location.pathname.includes("home.html")){
   const session = sessionStorage.getItem('session')
   console.log(session);
   
   if(!session){
     window.location.assign("index.html");
   }
   else{
    setInterval(() => {
      checkSessionValidity();
    }, 5000);
   }
  }
 }