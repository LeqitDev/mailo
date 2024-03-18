<script lang="ts">
	import { Button } from "@/components/ui/button";
	import { expandedSidenav } from '@/store';

    export let site: string;
    export let name: string;
    export let site_name: string;
    export let href: string;
    export let clicked: () => void = () => {};
    export let size: "default" | "icon" | "sm" = "default";
    export let active_variant: "default" | "ghost" | "secondary" = "default";
</script>

{#if $expandedSidenav}
    <Button variant={site === site_name ? active_variant : "ghost"} {href} {size} class="justify-start gap-2" on:click={clicked}><slot /> {name}</Button>
{:else}
    <Button variant={site === site_name ? active_variant : "ghost"} {href} size="icon" on:click={clicked}><slot /></Button>
{/if}