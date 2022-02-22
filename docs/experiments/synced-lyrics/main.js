// It's a beautiful day

seamless.polyfill()

function initialize() {
  const lyrics = document.querySelector('#lyrics>ul')
  const audioPlayer = document.querySelector('.audio-player')

  const sitPoint = lyrics.getBoundingClientRect().top

  let currentLine;
  audioPlayer.addEventListener('timeupdate', function (event) {
    const {target: {currentTime}} = event
    const currentPosition = parseInt(currentTime * 1000)

    // Use binary search
    let [start, end] = [0, lyrics.children.length]
    let found

    while (!found) {
      const mid = lyrics.children[parseInt((start + end) / 2)]
      if (start === end || end - start === 1) {
        found = mid
      } else if (mid._time > currentPosition) {
        end = mid._index
      } else {
        start = mid._index
      }
    }

    if (found !== currentLine) {
      currentLine = found
      lyrics.querySelector('.active')?.classList.toggle('active')
      found.classList.add('active')

      // This didn't behave correctly in Safari.
      //
      // document.querySelector('#lyrics')
      //   .scroll({top: found._top - 70, behavior: "smooth"})

      // Use polyfill instead
      // We can remove this when Safari supports smooth scroll
      const scrollBy = found.getBoundingClientRect().top - sitPoint
      seamless.scrollBy(document.querySelector('#lyrics'), {top: scrollBy, behavior: "smooth"})
    }
  })

  // Annotate lyric lines with integer time
  let index = 0
  for (let line of lyrics.children) {
    line._time = parseInt(line.dataset.time)
    line._index = index++
    line._top = line.getBoundingClientRect().top

    line.addEventListener('click', function () {
      audioPlayer.currentTime = line._time / 1000
    })
  }
}

initialize()
