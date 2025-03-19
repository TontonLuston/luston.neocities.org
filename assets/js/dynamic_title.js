const currentLocation = window.location;
console.log(currentLocation);
document.getElementById("path").innerHTML = "<a href=https://" + currentLocation.hostname + "/ id=host>" + currentLocation.hostname  + "</a>" + currentLocation.pathname;
