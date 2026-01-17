import * as esbuild from "esbuild";

esbuild.build({
    entryPoints: ["src/index.ts"],
    outdir: "dist",
    bundle: true,
    format: "esm",
    platform: "browser"
});