console.log('Installing Publis binary...')
let exec = require('child_process').exec

exec('curl -LSfs https://japaric.github.io/trust/install.sh | \
sh -s -- --git mobility46/publis', (error, stdout, stderr) => {
  console.log(stderr)
})


