<script lang="ts">
  import EnumSelect from "$lib/components/EnumSelect.svelte";
  import { Slider } from "$lib/components/ui/slider";
  import Label from "$lib/components/ui/label/label.svelte";
  import { AllFlag, CATPPUCCIN } from "$lib/flag";
  import defaultImg from "../../input.webp";
  import {
    applyOverlay,
    applyRing,
    type Flag,
    PresetFlag,
  } from "pride-overlay";

  enum EffectType {
    Overlay,
    Ring,
  }

  // state
  let flag: AllFlag = $state(AllFlag.Transgender);
  let effect = $state(EffectType.Overlay);
  let opacity = $state(0.5);
  let thickness = $state(0.1);

  let inputBuffer = $state(
    await fetch(defaultImg)
      .then((res) => res.arrayBuffer())
      .then((buf) => new Uint8Array(buf)),
  );
  let inputUrl = $derived(URL.createObjectURL(new Blob([inputBuffer])));

  let outputData = $derived(applyEffect(inputBuffer));
  let outputUrl = $derived(URL.createObjectURL(outputData));

  function applyEffect(inputData: Uint8Array) {
    let flagData: Flag;
    switch (flag) {
      case AllFlag.Catppuccin:
        flagData = CATPPUCCIN;
        break;
      default:
        flagData = flag;
        break;
    }

    let out: Uint8Array;
    switch (effect) {
      case EffectType.Overlay:
        out = applyOverlay(inputData, flagData, opacity);
        break;
      case EffectType.Ring:
        out = applyRing(inputData, flagData, opacity, thickness);
        break;
    }

    return new Blob([new Uint8Array(out.buffer as ArrayBuffer)])
  }
</script>

<h1 class="font-mono">pride-overlay</h1>

<form>
  <EnumSelect label="Flag" enum={AllFlag} bind:value={flag} />
  <EnumSelect label="Effect" enum={EffectType} bind:value={effect} />
  <Label for="opacity" class="font-bold">Opacity</Label>
  <Slider
    id="opacity"
    class="w-1/8"
    type="single"
    max={1}
    step={0.01}
    bind:value={opacity}
  />
  {#if effect === EffectType.Ring}
    <Label for="thickness" class="font-bold">Thickness</Label>
    <Slider
      id="thickness"
      class="w-1/8"
      type="single"
      max={1}
      step={0.01}
      bind:value={thickness}
    />
  {/if}
</form>

<div class="flex gap-24">
  <div class="text-center">
    <h2>Input</h2>
    <img src={inputUrl} alt="" height={128} />
  </div>
  <div class="text-center">
    <h2>Output</h2>
    <img src={outputUrl} alt="" height={128} />
  </div>
</div>
