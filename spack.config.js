const { config } = require('@swc/core/spack')


module.exports = config({
    entry: {
        'ui': __dirname + '/ui/index.tsx',
    },
    output: {
        path: __dirname + '/dist'
    },
    target: 'browser',
    mode: 'production'
});