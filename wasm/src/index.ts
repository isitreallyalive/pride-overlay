import initDecode, { WebPModule as DecodeModule } from "../squoosh/webp_dec";

let decodeModule: Promise<DecodeModule> | null = null;

async function getDecode(): Promise<DecodeModule> {
    if (!decodeModule) {
        decodeModule = initDecode({
            locateFile: (path: string) => {
                return `${new URL("../../../../", import.meta.url)}${path}`;
            }
        });
    }
    return decodeModule!;
}

export async function decode(data: Uint8Array): Promise<ImageData | null> {
    const decoder = await getDecode();
    const decoded = decoder.decode(data.buffer as ArrayBuffer);
    return decoded;
}