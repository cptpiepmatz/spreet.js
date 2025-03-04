"use strict";
import path from "node:path";
import fs from "node:fs/promises";

// @ts-types="../imports/spreet_js_imports.d.ts"
import { Options, spreet_impl, SpriteSvg } from "../imports/spreet_js_imports.js";
export type { Options };

export async function spreet(input: string, output: string, options?: Options) {
  const files = await fs.readdir(input);
  const spriteSvgs: SpriteSvg[] = [];
  for (const file of files.filter(file => file.endsWith(".svg"))) {
    const name = file.split(".svg")[0];
    const buffer = await fs.readFile(path.join(input, file));
    const content = new Uint8Array(buffer);
    spriteSvgs.push({name, content});
  }

  const dir = path.dirname(output);
  await fs.mkdir(dir, { recursive: true });

  // TODO: handle error better
  const spreetOutput = spreet_impl(spriteSvgs, options);

  await fs.writeFile(output + ".json", spreetOutput.json);
  await fs.writeFile(output + ".png", spreetOutput.png);
}
