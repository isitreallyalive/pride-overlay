import "./app.css";
import { PrideFlag, Overlay, Ring } from "pride-overlay";
import { canvas, drawImage } from "./canvas";

// current image data
let currentPixels: Uint8Array | null = null;
let currentWidth = 0;
let currentHeight = 0;

// instantiate the effects with default values (using same flag for both)
let currentFlag = PrideFlag.Agender
let overlay = new Overlay(currentFlag, 0.4);
let ring = new Ring(currentFlag, 1, 24);

// function to get PrideFlag enum value from string
function getPrideFlagFromString(flagName: string): PrideFlag {
  return PrideFlag[flagName as keyof typeof PrideFlag];
}

// function to re-render effects when image or flags change
function renderEffects() {
  if (!currentPixels) return;

  // render overlay
  const overlayPixels = overlay.apply(
    currentPixels,
    currentWidth,
    currentHeight
  );
  drawImage(canvas.overlay, currentWidth, currentHeight, overlayPixels);

  // render ring
  const ringPixels = ring.apply(currentPixels, currentWidth, currentHeight);
  drawImage(canvas.ring, currentWidth, currentHeight, ringPixels);
}

// listen for flag changes
document
  .querySelector<HTMLSelectElement>("select#prideFlag")!
  .addEventListener("change", ({ target }) => {
    const selectedFlag = (target as HTMLSelectElement).value;
    const flagEnum = getPrideFlagFromString(selectedFlag);

    // free old effects
    overlay.free();
    ring.free();

    // update current flag and recreate both effects
    currentFlag = flagEnum;
    overlay = new Overlay(currentFlag, 0.4);
    ring = new Ring(currentFlag, 1, 24);
    renderEffects();
  });

// listen for file uploads
document
  .querySelector<HTMLInputElement>("input#upload")!
  .addEventListener("change", async ({ target }) => {
    const file = (target as HTMLInputElement).files?.[0];
    if (!file) return;

    // make sure the file is an image
    if (!file.type.startsWith("image/")) {
      alert("Please upload a valid image file.");
      return;
    }

    try {
      const imageBitmap = await createImageBitmap(file);
      const { width, height } = imageBitmap;

      // store current image data for re-rendering
      currentPixels = await getPixelsFromImageBitmap(imageBitmap);
      currentWidth = width;
      currentHeight = height;

      // original
      drawImage(canvas.original, width, height, currentPixels);

      // render effects
      renderEffects();
    } catch (error) {
      console.error("Error processing image:", error);
    }
  });

async function getPixelsFromImageBitmap(
  imageBitmap: ImageBitmap
): Promise<Uint8Array> {
  const canvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
  const ctx = canvas.getContext("2d");

  if (!ctx) throw new Error("Could not get 2D context");

  ctx.drawImage(imageBitmap, 0, 0);
  const imageData = ctx.getImageData(
    0,
    0,
    imageBitmap.width,
    imageBitmap.height
  );

  return new Uint8Array(imageData.data);
}
