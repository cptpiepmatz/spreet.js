"use strict";
import url from "url";
import path from "path";
import { WASI } from "wasi";
import fs from "fs/promises";

/**
 * @typedef {object} PluginOptions
 * @property {string} input
 * @property {string} output
 * @property {number} [ratio]
 * @property {boolean} [retina]
 * @property {boolean} [unique]
 * @property {boolean} [minifyIndexFile]
 * @property {boolean} [sdf]
 */

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const __wasm = path.join(__dirname, "esbuild_plugin_spreet.wasm");

/**
 * @param {PluginOptions} options
 * @returns {import("esbuild").Plugin}
 */
export default function plugin(options) {
  return {
    name: "esbuild-plugin-spreet",
    /**
     * 
     * @param {import("esbuild").PluginBuild} _build 
     */
    async setup(_build) {
      if (!options.input) throw new Error("missing input");
      if (!options.output) throw new Error("missing output");

      let files = (await fs.readdir(options.input)).filter(f => f.endsWith(".svg"));
      let dir = path.dirname(options.output);
      await fs.mkdir(dir, {recursive: true});
      const wasi = new WASI({
        version: "preview1",
        preopens: {
          "/input": options.input,
          "/output": dir,
        }
      });
      const wasmBuffer = await fs.readFile(__wasm);
      const wasm = await WebAssembly.compile(wasmBuffer);
      const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

      wasi.start(instance);

      let optionsJson = JSON.stringify({
        ...options,
        files,
      });
      let optionsPtr = instance.exports.alloc_string(optionsJson.length);
      
      const memory = new Uint8Array(instance.exports.memory.buffer);
      const encoder = new TextEncoder();
      const encoded = encoder.encode(optionsJson);
      memory.set(encoded, optionsPtr);
      
      instance.exports.plugin(optionsPtr, optionsJson.length);
    }
  }
}
