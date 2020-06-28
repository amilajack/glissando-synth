import { wasm } from "@rollup/plugin-wasm";

export default {
  input: "./index.js",
  output: {
    file: "dist/glissando-synth.js",
    format: "module",
  },
  plugins: [
    wasm({
      sync: ["./pkg/glissando_synth_bg.wasm"],
    }),
  ],
};
