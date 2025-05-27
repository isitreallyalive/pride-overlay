import "./app.css";
import { PrideFlag, Overlay, Ring } from "pride-overlay";
import { canvas, drawImage } from "./canvas";

// instantiate the effects
const lesbianOverlay = new Overlay(PrideFlag.Lesbian, 0.4);
const transgenderRing = new Ring(PrideFlag.Transgender, 0.4, 24);

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

    // read the file
    const reader = new FileReader();
    reader.onload = (e) => {
      const img = new Image();
      img.onload = () => {
        drawImage(canvas.original, img);
      };
      img.src = e.target?.result as string;
    };

    reader.readAsDataURL(file);
  });
