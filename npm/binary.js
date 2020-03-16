const axios = require('axios')
const fs = require('fs')
const Stream = require('stream').Transform


const download = async (path, dest) => {

  const response = await axios({
    url: `https://github.com${path}`,
    responseType: 'arraybuffer'
  })


  fs.writeFileSync(dest, response.data)
  fs.chmodSync(dest, 0o755) 
}

module.exports = { download }
