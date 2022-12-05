async function signup() {
  // console.log("sdfaksdjf");
  // let username = document.getElementById("username").value;
  // document.getElementById("username").innerHTML = "";
  // console.log(username);
  // // console.log(kkk

  // let formData = new FormData();
  formData = [];
  formData.push("id=" + "1");
  fetch("https://localhost:3000/signup", {
    method: "POST",
    body: formData,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
  })
    .then(data => {
      console.log(data);
    })
    .catch(error => {
      document.getElementById("error").innerHTML = error;
    });
}

async function getAlbumTitle() {
  document.getElementById("error").innerHTML = "";
  document.getElementById("result").innerHTML = "";

  let id = document.getElementById("album-id").value;
  fetch("https://127.0.0.1:3000/albums/" + id, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  })
    .then(response => response.json())
    .then(data => {
      document.getElementById("result").innerHTML = data[0].title;
    })
    .catch(error => {
      document.getElementById("error").innerHTML = error;
    });
}

// o
//   const XHR = new XMLHttpRequest();
//   const FD = new FormData();

//   // Push our data into our FormData object
//   FD.append("username", username);
//   // Define what happens on successful data submission
//   XHR.addEventListener("load", event => {
//     alert("Yeah! Data sent and response loaded.");
//   });

//   // Define what happens in case of error
//   XHR.addEventListener("error", event => {
//     alert("Oops! Something went wrong.");
//   });

//   // Set up our request
//   XHR.open("POST", "https://localhost:3000/signup");

//   // Send our FormData object; HTTP headers are set automatically
//   XHR.send(FD);
