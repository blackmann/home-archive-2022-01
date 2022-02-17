import fs from 'fs'
import {parse} from "node-html-parser";

function parseTime(timeString) {
  const [minute, seconds] = timeString.split(':').map((n) => parseFloat(n))

  return (minute * 60 * 1000) + seconds * 1000
}

const LYRIC_LINE_REG = /\[(\d{2}:\d{2}\.\d{2})](.*)/

const [lrcFile, outHtml] = process.argv.slice(2)
const lrcContent = fs.readFileSync(lrcFile, 'utf8')

const lines = lrcContent
  .split('\n')
  .filter((line) => LYRIC_LINE_REG.test(line))
  .map((line) => {
    const lineMatch = line.match(LYRIC_LINE_REG)
    return {time: parseTime(lineMatch[1]), line: lineMatch[2]}
  })


const lyricLines = lines.map((line) => (`<li data-time="${line.time}">${line.line}</li>`)).join('\n')

const htmlContent = fs.readFileSync(outHtml, 'utf8')

const html = parse(htmlContent)

const lyricsEl = html.querySelector('#lyrics')

lyricsEl.innerHTML = `
<ul>${lyricLines}</ul>
`

fs.writeFileSync(outHtml, html.toString())
