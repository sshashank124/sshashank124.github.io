function section_click(event, name) {
  var buttons = document.getElementsByClassName("section-button");
  var sections = document.getElementsByClassName("section");

  var i;
  for (i = 0; i < sections.length; i++) {
    buttons[i].classList.remove("selected");
    sections[i].style.display = "none";
  }

  document.getElementById(name).style.display = "block";

  var button = event.currentTarget;
  button.classList.add("selected");
  button.blur();
}
