import init from "../dist/webp";

const webp = await init({
    locateFile: (path: string) => {
        return `${new URL("../../../../", import.meta.url)}${path}`;
    }
});

export function decode(data: Uint8Array): ImageData | null {
    return webp.decode(data);
}

export function encode(data: Uint8Array, width: number, height: number): Uint8Array | null {
    return webp.encode(data, width, height);
}