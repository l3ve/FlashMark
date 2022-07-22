const path = require("path");

module.exports = {
  entry: {
    home: "./ui/home/index.tsx",
    add: "./ui/add/index.tsx",
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "dist"),
    publicPath: "/",
  },
  optimization: {
    splitChunks: {
      cacheGroups: {
        default: {
          name: "common",
          chunks: "initial",
          minChunks: 2  //模块被引用2次以上的才抽离
        },
      },
    },
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.(js|ts|tsx)$/,
        exclude: /(node_modules)/,
        use: {
          // `.swcrc` can be used to configure swc
          loader: "swc-loader",
        },
      },
    ],
  },
  resolve: {
    extensions: ["*", ".js", ".ts", ".tsx"],
    modules: [path.resolve(__dirname, "./ui"), "node_modules"],
  },
};
