const fragment_lists = document.querySelectorAll(
  "ul[fragments], ol[fragments]"
);
for (const fragment_list of fragment_lists) {
  fragment_list.attributes.removeNamedItem("fragments");
  let inner = fragment_list.querySelector("ul, ol");
  for (const li of inner.querySelectorAll("li")) {
    console.log(li);
    li.className = "fragment";
  }
  inner.outerHTML = inner.innerHTML;
}
