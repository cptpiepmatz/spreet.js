import plugin from "../dist/plugin.mjs";
import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'fs/promises';
import path from 'path';

const outputDir = "tests/output";
const jsonFile = path.join(outputDir, "sprites.json");
const pngFile = path.join(outputDir, "sprites.png");

async function removeFiles() {
    await fs.rm(jsonFile, { force: true });
    await fs.rm(pngFile, { force: true });
}

test("plugin throws error if input is missing", async () => {
    const pluginInstance = plugin({
        // @ts-ignore we want to force invalid input here
        input: null,
        output: "tests/output",
    });

    await assert.rejects(pluginInstance.setup(), Error);
});

test("plugin throws error if output is missing", async () => {
    const pluginInstance = plugin({
        input: "tests/input",
        // @ts-ignore we want to force invalid input here
        output: null,
    });

    await assert.rejects(pluginInstance.setup(), Error);
});

test("plugin generates expected files", async () => {
    await removeFiles();

    const pluginInstance = plugin({
        input: "tests/input",
        output: `${outputDir}/sprites`,
    });

    await pluginInstance.setup();

    // Check that the files exist
    await assert.doesNotReject(fs.access(jsonFile));
    await assert.doesNotReject(fs.access(pngFile));

    // Read and verify JSON content
    const jsonContent = JSON.parse(await fs.readFile(jsonFile, "utf-8"));
    assert.deepStrictEqual(Object.keys(jsonContent).sort(), ["deno-primary", "deno-secondary"]);
});

