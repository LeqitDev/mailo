<script lang="ts">
	import CustomLayout from "@/custom/layouts/_customLayout.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

    <h1>{data.slug}</h1>