const { config } = require('@swc/core/spack')


module.exports = config({
    entry: {
        'ui': __dirname + '/ui/index.ts',
    },
    output: {
        path: __dirname + '/dist'
    },
    mode: 'debug',
});