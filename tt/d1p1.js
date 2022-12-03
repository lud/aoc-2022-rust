const fs = require('fs')

const text = fs.readFileSync("src/inputs/01.txt").toString()

const groups = text
  .trim()
  .split("\n\n")
  .map(group => group
    .split("\n")
    .map(s => parseInt(s))
    .reduce((acc, v) => acc + v))

const p1 = Math.max(...groups)

console.log(`p1`, p1)

