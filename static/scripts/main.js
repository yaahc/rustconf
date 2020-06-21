let SLIDE_PREFIX = "#slide-"

var current_slide = (() => {
    let hash = window.location.hash;
    if (hash.startsWith(SLIDE_PREFIX)) {
        return +hash.substring(SLIDE_PREFIX.length)
    } else {
        return 1
    }
})()

let slide_exists = num => !!document.getElementById(`slide-${num}`)

function next_slide() {
    if (slide_exists(current_slide + 1)) {
        window.location.hash = SLIDE_PREFIX + ++current_slide
    }
}

function prev_slide() {
    if (slide_exists(current_slide - 1)) {
        window.location.hash = SLIDE_PREFIX + --current_slide
    }
}

document.addEventListener("keyup", e => {
    if (e.key == "j" || e.key == "ArrowDown") {
        next_slide()
    } else if (e.key == "k" || e.key == "ArrowUp") {
        prev_slide()
    }
})
