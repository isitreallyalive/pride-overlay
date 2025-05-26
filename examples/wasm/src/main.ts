import { PrideFlag, Overlay } from "pride-overlay";

const effect = new Overlay(PrideFlag.Lesbian, 0.3);

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>WASM Example</h1>
  </div>
`
