async function signup() {
  let username = document.getElementById("username").value;
  formData = [];
  formData.push("username=" + username);

  fetch("https://localhost:3000/signup", {
    method: "POST",
    body: formData,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
  })
    .then(response => response.json())
    .then(data => {
      document.getElementById("token").innerHTML = data.token;
    })
    .catch(error => {
      document.getElementById("error").innerHTML = error;
    });
}

async function getAlbumTitle() {
  document.getElementById("error").innerHTML = "";
  document.getElementById("result").innerHTML = "";
  let token = document.getElementById("token").innerHTML;
  let id = parseInt(document.getElementById("album-id").value);
  fetch("https://localhost:3000/api/request", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: token,
    },
    body: JSON.stringify({
      id: id,
    }),
  })
    .then(response => response.json())
    .then(data => {
      document.getElementById("result").innerHTML = data[0].title;
    })
    .catch(error => {
      document.getElementById("error").innerHTML = "Error with token";
    });
}
