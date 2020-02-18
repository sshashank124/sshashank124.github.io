var last_id,
    header_height = $("#header").outerHeight(),
    headers = $("#sections").find("a"),
    sections = headers.map(function() { return $($(this).attr("href")); });

// Highlight current header
$(window).scroll(function() {
  var position = $(this).scrollTop() + header_height + 1;

  var cur = sections.map(function() {
    if ($(this).offset().top < position) return this;
  });
  cur = cur[cur.length - 1];

  var id = cur ? cur[0].id : "";

  if (last_id !== id) {
    last_id = id;
    headers.removeClass("selected")
           .filter("[href='#"+id+"']").addClass("selected");
  }
});

// Smooth scroll animation
headers.click(function(e) {
  var href = $(this).attr("href"),
      position = href === "#" ? 0 : $(href).offset().top - header_height;
  $('html, body').stop().animate({ scrollTop: position }, 120);
  e.preventDefault();
});
