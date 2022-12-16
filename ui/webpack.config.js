const path = require('path')
const WebpackObfuscator = require('webpack-obfuscator')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
module.exports = {
    entry: './src/index.js',
    devtool: 'inline-source-map',
    output: {
        filename: 'app.js',
        path: path.resolve('../server/public')
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                use: 'babel-loader',
                exclude: /node_modules/,
            },
            {
                test: /(@?react-designer).*\.(ts|js)x?$/,
                include: /node_modules/,
                exclude: [/react-native-web/, /\.(native|ios|android)\.(ts|js)x?$/],
                loader: 'babel-loader'
            },
            {
                test: /\.css$/,
                use: [
                    'style-loader',
                    'css-loader',
                ]
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
                test: /\.(ttf|jpg)$/i,
                type: 'asset/resource',
            },
        ]
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: 'app.css'
        }),
        new HtmlWebpackPlugin({
            template: './src/index.html',
            favicon: './src/favicon.ico'
        }),
        new WebpackObfuscator({
            rotateStringArray: true,
        }, ['app.js']),
    ],
    watchOptions: {
        ignored: ['dist/**', 'node_modules/**'],
        poll: 1000,
        aggregateTimeout: 600
    }
}