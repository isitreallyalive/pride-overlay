<script lang="ts">
  import { onMount } from "svelte";
  import EnumSelect from "$lib/components/EnumSelect.svelte";
  import { Slider } from "$lib/components/ui/slider";
  import defaultImg from "../../input.webp";

  import { Flags, applyOverlay, applyRing } from "pride-overlay";
  import Label from "$lib/components/ui/label/label.svelte";

  enum Effect {
    Overlay,
    Ring,
  }

  // image elements
  let beforeImg!: HTMLImageElement;
  let afterImg!: HTMLImageElement;

  // state
  let flag = $state(Flags.Transgender);
  let effect = $state(Effect.Overlay);
  let opacity = $state(0.5);
  let thickness = $state(0.1);

  // reusable offscreen canvas to avoid allocations on each conversion
  const _offscreen = document.createElement("canvas");
  const _ctx = _offscreen.getContext("2d");

  /**
   * Convert an HTMLImageElement into encoded image bytes (Uint8Array).
   */
  async function imageToUint8Array(img: HTMLImageElement): Promise<Uint8Array> {
    _offscreen.width = img.naturalWidth;
    _offscreen.height = img.naturalHeight;

    _ctx!.clearRect(0, 0, _offscreen.width, _offscreen.height);
    _ctx!.drawImage(img, 0, 0);

    const blob: Blob | null = await new Promise((resolve) =>
      _offscreen.toBlob(resolve, "image/png"),
    );
    if (!blob) throw new Error("failed to encode image to blob");

    const arrayBuffer = await blob.arrayBuffer();
    return new Uint8Array(arrayBuffer);
  }

  // keep track of the last object URL so we can revoke it when replaced
  let _currentObjectUrl: string | null = null;

  /**
   * Apply the given effect and flag to the before image, updating the after image.
   */
  async function applyEffect(
    effect: Effect,
    flag: Flags,
    opacity: number,
    thickness: number,
  ): Promise<void> {
    // call pride-overlay to apply the effect
    const beforeData = await imageToUint8Array(beforeImg);
    let afterData: Uint8Array;
    switch (effect) {
      case Effect.Overlay:
        afterData = applyOverlay(beforeData, flag, opacity);
        break;
      case Effect.Ring:
        afterData = applyRing(beforeData, flag, opacity, thickness);
        break;
    }

    // revoke previous URL to avoid leaking memory
    if (_currentObjectUrl) {
      URL.revokeObjectURL(_currentObjectUrl);
      _currentObjectUrl = null;
    }

    // update the after image
    const blob = new Blob([afterData.buffer as ArrayBuffer], {
      type: "image/png",
    });
    const url = URL.createObjectURL(blob);
    _currentObjectUrl = url;
    afterImg.src = url;
  }

  onMount(async () => {
    // set the default image
    beforeImg.src = defaultImg;
    beforeImg.onload = () => {
      applyEffect(effect, flag, opacity, thickness);
    };
  });

  $effect(() => {
    applyEffect(effect, flag, opacity, thickness);
  });

  $effect(() => {
    if (thickness < 0) thickness = 0;
  });
</script>

<h1 class="font-mono">pride-overlay</h1>

<form>
  <EnumSelect label="Flag" enum={Flags} bind:value={flag} />
  <EnumSelect label="Effect" enum={Effect} bind:value={effect} />
  <Label for="opacity" class="font-bold">Opacity</Label>
  <Slider
    id="opacity"
    class="w-1/8"
    type="single"
    max={1}
    step={0.01}
    bind:value={opacity}
  />
  {#if effect === Effect.Ring}
    <Label for="thickness" class="font-bold">Thickness</Label>
    <Slider
      id="thickness"
      class="w-1/8"
      type="single"
      max={1}
      step={0.01}
      bind:value={thickness} />
  {/if}
</form>

<div class="flex gap-24">
  <div class="text-center">
    <h2>Before</h2>
    <img bind:this={beforeImg} alt="" height={128} />
  </div>
  <div class="text-center">
    <h2>After</h2>
    <img bind:this={afterImg} alt="" height={128} />
  </div>
</div>
