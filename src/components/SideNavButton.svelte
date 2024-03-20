<script lang="ts">
	import { Button } from "@/components/ui/button";
	import { expandedSidenav } from '@/store';
	import type { ButtonTypes } from "@/utils";
	import { afterUpdate } from "svelte";

    export let site: string;
    export let name: string;
    export let site_name: string;
    export let href: string;
    export let clicked: () => void = () => {};
    export let size: "default" | "icon" | "sm" = "default";
    export let active_variant: ButtonTypes = "default";
    export let default_variant: ButtonTypes = "ghost";
    export let expanded: boolean = true;
    export let active_classes: string = "";

    $: active = site === site_name;
</script>

{#if expanded}
    <Button variant={active ? active_variant : default_variant} {href} {size} class={`justify-start gap-2 ${active ? active_classes : ""}`} on:click={clicked}><slot /> {name}</Button>
{:else}
    <Button variant={active ? active_variant : default_variant} {href} size="icon" class={active ? active_classes : ""} on:click={clicked}><slot /></Button>
{/if}