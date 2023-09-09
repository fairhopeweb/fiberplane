import dts from "rollup-plugin-dts";
import { defineRollupSwcOption, swc } from "rollup-plugin-swc3";

export default {
  input: "src/index.ts",
  output: {
    file: "dist/index.d.ts",
    format: "es",
  },
  plugins: [swc(defineRollupSwcOption({ sourceMaps: true })), dts()],
};
