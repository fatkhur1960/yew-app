const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')

const distPath = path.resolve(__dirname, 'dist')
module.exports = (env, argv) => {
  return {
    devServer: {
      historyApiFallback: true,
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
    },
    entry: { app: ['./bootstrap.js'] },
    output: {
      path: distPath,
      filename: 'yewapp.js',
      webassemblyModuleFilename: 'yewapp.wasm',
      publicPath: '/',
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ['style-loader', 'css-loader', 'sass-loader'],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{ from: './static', to: distPath }],
      }),
      new WasmPackPlugin({
        crateDirectory: '.',
        extraArgs: '--no-typescript',
      }),
    ],
    watch: argv.mode !== 'production',
  }
}
