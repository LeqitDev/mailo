<script>
    export let data;
	import EmailPreview from "$lib/components/EmailPreview.svelte";
	import { Button } from "@/components/ui/button";
	import { selected_previews } from "@/stores/email";
</script>
<div class="h-full flex flex-col">
    <div class="w-full border-b flex justify-between items-center px-2 py-1 gap-5 h-12">
        <h2 class="text-lg font-semibold capitalize">{data.slug}</h2>
        {#if $selected_previews.length > 0}
            <div class="flex gap-2 items-center">
                {#if $selected_previews.length > 1}
                    <p>Selected {$selected_previews.length} mails</p>
                {:else}
                    <p>Selected 1 mail</p>
                {/if}
                <Button variant="outline" size="sm" on:click={() => {}}>Mark as seen</Button>
                <Button variant="outline" size="sm" on:click={() => {}}>Delete</Button>
            </div>
        {/if}
    </div>
    <div class="flex-1 overflow-y-auto">
        {#each Array.from({ length: 100 }, (_, i) => i) as id}
            <EmailPreview id={id} />
        {/each}
    </div>
</div>