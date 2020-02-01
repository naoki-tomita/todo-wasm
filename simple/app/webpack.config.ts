import { Configuration } from "webpack";
import { resolve } from "path";

const config: Configuration = {
  entry: "./src/index.ts",
  output: {
    path: resolve(__dirname, "dist"),
    filename: "main.js",
  },
  mode: "development"
};

export default config;
