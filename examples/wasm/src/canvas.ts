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

export function drawImage(canvas: HTMLCanvasElement, image: HTMLImageElement) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  // scale to fit the square canvas while maintaining aspect ratio
  const scale = Math.min(canvasSize / image.width, canvasSize / image.height);
  const scaledWidth = image.width * scale;
  const scaledHeight = image.height * scale;

  // center the image in the square canvas
  const x = (canvasSize - scaledWidth) / 2;
  const y = (canvasSize - scaledHeight) / 2;

  // clear and draw the image
  ctx.clearRect(0, 0, canvasSize, canvasSize);
  ctx.drawImage(image, x, y, scaledWidth, scaledHeight);
}
