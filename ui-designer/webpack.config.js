const path = require('path')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

module.exports = {
    entry: './index.js',
    devtool: 'inline-source-map',
    output: {
        filename: 'designer.js',
        path: path.resolve('./dist')
    },
    module: {
        rules: [
            { 
                test: /\.js$/, 
                exclude: /node_modules/, 
                use: 'babel-loader',
            },
            {
                test: /\.less$/,
                use: [
                    'style-loader',
                    'css-loader',
                    'less-loader',
                ]
            },
            {
                test: /\.(ttf)$/i,
                type: 'asset/resource',
            },
        ]
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: 'designer.css'
        })
    ],
    watchOptions: {
        ignored: ['node_modules/**'],
        poll: 1000,
        aggregateTimeout: 600
    }
}