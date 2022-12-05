async function getAlbumTitle() {
  document.getElementById("error").innerHTML = "";
  document.getElementById("result").innerHTML = "";

  let id = document.getElementById("album-id").value;
  fetch("http://localhost:3000/albums/" + id, {
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
