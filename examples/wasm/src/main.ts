import "./style.css";
import inputPath from "../../input.webp";
import { apply, Effect, Flags } from "pride-overlay";

const app = document.querySelector<HTMLDivElement>('#app')!;

app.innerHTML = `
  <div class="controls">
    <label>Upload image: <input id="file" type="file" accept="image/*"></label>
    <label>Effect:
      <select id="effect">
        <option value="${Effect.Overlay}">Overlay</option>
        <option value="${Effect.Ring}">Ring</option>
      </select>
    </label>
    <label>Flag:
      <select id="flag"></select>
    </label>
    <button id="run">Process</button>
  </div>
  <div class="images">
    <div>
      <h4>Original</h4>
      <img id="orig" />
    </div>
    <div>
      <h4>Processed</h4>
      <img id="out" />
    </div>
  </div>
`;

// populate flag select from the exported Flags enum (object with both names and numeric keys)
const flagSelect = document.getElementById('flag') as HTMLSelectElement;
for (const k of Object.keys(Flags)) {
  // skip numeric keys
  if (!Number.isNaN(Number(k))) continue;
  const opt = document.createElement('option');
  opt.value = (Flags as any)[k];
  opt.textContent = k;
  flagSelect.appendChild(opt);
}

const origImg = document.getElementById('orig') as HTMLImageElement;
const outImg = document.getElementById('out') as HTMLImageElement;

// show bundled example input by default
origImg.src = inputPath;

const fileInput = document.getElementById('file') as HTMLInputElement;
fileInput.addEventListener('change', async () => {
  const f = fileInput.files && fileInput.files[0];
  if (!f) return;
  const url = URL.createObjectURL(f);
  origImg.src = url;
});

document.getElementById('run')!.addEventListener('click', async () => {
  const f = fileInput.files && fileInput.files[0];
  // if no uploaded file, fetch the bundled example image
  let bytes: Uint8Array;
  if (f) {
    const buf = await f.arrayBuffer();
    bytes = new Uint8Array(buf);
  } else {
    const buf = await fetch(inputPath).then(r => r.arrayBuffer());
    bytes = new Uint8Array(buf);
  }

  const effect = Number((document.getElementById('effect') as HTMLSelectElement).value) as Effect;
  const flag = Number((document.getElementById('flag') as HTMLSelectElement).value) as Flags;

  // call into wasm apply (returns Uint8Array)
  const out = apply(bytes, effect, flag as any);

  // create blob and set preview
  const blob = new Blob([out.buffer], { type: 'image/webp' });
  outImg.src = URL.createObjectURL(blob);
});
