const path = require('path')
const { download } = require('./binary')

const asset = '/mobility46/publis/releases/download/v0.2.0/publis'

download(asset, path.resolve(__dirname, '..', 'bin', 'publis'))
