console.log('Installing Publis binary...')
let exec = require('child_process').exec

const url = 'https://github.com/mobility46/release/latest/download/latest/publis'

exec(`curl -o publis ${url}`, (error, stdout, stderr) => {
  console.log(stderr)
})


