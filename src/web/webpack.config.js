const path = require('path');

module.exports = {
    // entry: './src/index.ts',
//    entry: './src/command.ts',
//    entry: './src/command_wave.ts',
//     entry: './src/command_v11.ts',
    entry: './src/command_v12.ts',
    mode: 'development', //'production'
    devtool: 'inline-source-map',
    watch: true,
    watchOptions: {
        aggregateTimeout: 200,
        poll: 1000,
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
};