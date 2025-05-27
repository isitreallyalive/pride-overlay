const canvasSize = 256;

function getCanvas(id: string): HTMLCanvasElement {
  const canvas = document.querySelector<HTMLCanvasElement>(`canvas#${id}`)!;
  canvas.width = canvasSize;
  canvas.height = canvasSize;
  return canvas;
}

export const canvas = {
  original: getCanvas("original"),
  overlay: getCanvas("overlay"),
  ring: getCanvas("ring"),
};

export function drawImage(
  canvas: HTMLCanvasElement,
  width: number,
  height: number,
  pixels: Uint8Array
) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const image = new ImageData(new Uint8ClampedArray(pixels), width, height);

  // create a temporary canvas to draw the image data
  const tmpCanvas = document.createElement("canvas");
  const tmpCtx = tmpCanvas.getContext("2d");
  if (!tmpCtx) return;

  tmpCanvas.width = width;
  tmpCanvas.height = height;
  tmpCtx.putImageData(image, 0, 0);

  // scale to fit the square canvas while maintaining aspect ratio
  const scale = Math.min(canvasSize / width, canvasSize / height);
  const scaledWidth = width * scale;
  const scaledHeight = height * scale;

  // center the image in the square canvas
  const x = (canvasSize - scaledWidth) / 2;
  const y = (canvasSize - scaledHeight) / 2;

  // clear and draw the scaled image
  ctx.clearRect(0, 0, canvasSize, canvasSize);
  ctx.drawImage(tmpCanvas, x, y, scaledWidth, scaledHeight);
}
