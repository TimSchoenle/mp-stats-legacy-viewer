// Copies the self-hosted font files out of `node_modules` and into `public/fonts`.
//
// `dx` has no directive for pulling a file out of `node_modules`, but it copies everything under
// `public/` into the root of its output verbatim, so the destination below is the URL: a file
// written to
// `public/fonts/inter-latin-400-normal.woff2` is served at `/fonts/inter-latin-400-normal.woff2`,
// which is what the `@font-face` rules in `assets/input.css` already ask for.
//
// Written in Node rather than as a `cp` in an npm script because this repository is developed on
// Windows and built on Linux, and `cp` exists on only one of them. Node is already a build
// dependency - it is what runs the Tailwind CLI - so this costs no new tool.

import { mkdir, copyFile } from "node:fs/promises";
import { dirname, join, basename } from "node:path";
import { fileURLToPath } from "node:url";

const frontendDir = dirname(dirname(fileURLToPath(import.meta.url)));
const modulesDir = join(frontendDir, "node_modules");
const outDir = join(frontendDir, "public", "fonts");

// Only the latin subsets, and only the weights the stylesheet declares. The variable faces
// (`-wght-`) carry their whole weight range in one file, which is why Geist and JetBrains Mono
// need one entry each where Inter needs two.
const fonts = [
  "@fontsource/inter/files/inter-latin-400-normal.woff2",
  "@fontsource/inter/files/inter-latin-700-normal.woff2",
  "@fontsource-variable/geist/files/geist-latin-wght-normal.woff2",
  "@fontsource-variable/jetbrains-mono/files/jetbrains-mono-latin-wght-normal.woff2",
  "@fontsource/instrument-serif/files/instrument-serif-latin-400-normal.woff2",
  "@fontsource/instrument-serif/files/instrument-serif-latin-400-italic.woff2",
];

await mkdir(outDir, { recursive: true });

await Promise.all(
  fonts.map(async (font) => {
    const from = join(modulesDir, font);
    const to = join(outDir, basename(font));

    // A missing file means `npm install` has not run, or a @fontsource package renamed a subset
    // between versions. Both are worth failing the build over: the alternative is a site that
    // builds clean and renders in the fallback font.
    await copyFile(from, to).catch((cause) => {
      throw new Error(`could not copy ${font} - run \`npm install\` in apps/frontend first`, {
        cause,
      });
    });
  }),
);

console.log(`copied ${fonts.length} font files into public/fonts`);
