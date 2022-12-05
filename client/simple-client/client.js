async function getAlbumTitle() {
  document.getElementById("error").innerHTML = "";
  document.getElementById("result").innerHTML = "";

  fetch("http://localhost:3000/albums/" + id, {
    method: "GET",
  })
    .then(response => response.json())
    .then(data => {
      document.getElementById("result").innerHTML = data[0].title;
    })
    .catch(error => {
      document.getElementById("error").innerHTML = error;
    });
}
