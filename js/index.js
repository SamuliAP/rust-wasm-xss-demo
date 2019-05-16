import("../crate/pkg").then(module => {
  console.log("<script>alert(1)</script>")
  console.log(module.sanitize_html("<script>alert(1)</script>"))
});
