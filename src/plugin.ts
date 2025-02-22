"use strict";
import url from "node:url";
import path from "node:path";
import { WASI } from "node:wasi";
import fs from "node:fs/promises";
import type { Plugin, PluginBuild } from "npm:esbuild";

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const __wasm = path.join(__dirname, "esbuild_plugin_spreet.wasm");

export interface PluginOptions {
  input: string;
  output: string;
  ratio?: number;
  retina?: boolean;
  unique?: boolean;
  minifyIndexFile?: boolean;
  sdf?: boolean;
}

interface WasmExports {
  alloc_string: (len: number) => number;
  plugin: (buf: number, len: number) => void;
  memory: WebAssembly.Memory;
}

export default function plugin(options: PluginOptions): Plugin {
  return {
    name: "esbuild-plugin-spreet",
    async setup(_build: PluginBuild) {
      if (!options.input) throw new Error("missing input");
      if (!options.output) throw new Error("missing output");

      const files = (await fs.readdir(options.input)).filter((f) =>
        f.endsWith(".svg")
      );
      const dir = path.dirname(options.output);
      await fs.mkdir(dir, { recursive: true });
      const wasi = new WASI({
        version: "preview1",
        preopens: {
          "/input": options.input,
          "/output": dir,
        },
      });
      const wasmBuffer = await fs.readFile(__wasm);
      const wasm = await WebAssembly.compile(wasmBuffer);
      const instance = await WebAssembly.instantiate(wasm);

      wasi.start(instance);
      const exports = instance.exports as unknown as WasmExports;

      const optionsJson = JSON.stringify({
        ...options,
        files,
      });
      const optionsPtr = exports.alloc_string(optionsJson.length);

      const memory = new Uint8Array(exports.memory.buffer);
      const encoder = new TextEncoder();
      const encoded = encoder.encode(optionsJson);
      memory.set(encoded, optionsPtr);

      exports.plugin(optionsPtr, optionsJson.length);
    },
  };
}
