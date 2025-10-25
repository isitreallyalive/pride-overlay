<script lang="ts">
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";

  interface Props {
    label: string;
    enum: Record<string, string>;
    value: string;
  }

  let { label, enum: enumObj, value = $bindable() } = $props() as Props;
  const id = label.toLowerCase();
  const entries = Object.entries(enumObj).filter(([k, _]) => isNaN(Number(k)));
</script>

<div>
  <Label for={id} class="font-bold">{label}</Label>
  <Select.Root {id} type="single" bind:value>
    <Select.Trigger>{enumObj[value]}</Select.Trigger>
    <Select.Content>
      {#each entries as [key, value]}
        <Select.Item {value}>{key}</Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
