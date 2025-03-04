import fs from "node:fs/promises";
import path from "node:path";
import {spreet} from "../src/lib.ts";
import { expect } from "jsr:@std/expect";

const input = "e2e/input";
const output = "e2e/output/sprites";
const jsonFile = output + ".json";
const pngFile = output + ".png";

Deno.test({
  name: "spreet generates expected files",
  permissions: {read: true, write: true},
  fn: async () => {
    await fs.rm(jsonFile, {force: true});
    await fs.rm(pngFile, {force: true});

    await spreet(input, output, {prettyJson: true});

    expect(fs.access(jsonFile)).resolves.not.toThrow();
    expect(fs.access(pngFile)).resolves.not.toThrow();

    const jsonContent = JSON.parse(await fs.readFile(jsonFile, "utf-8"));
    expect(Object.keys(jsonContent).sort()).toStrictEqual(["deno-primary", "deno-secondary"]);
  }
});