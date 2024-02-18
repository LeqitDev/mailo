<script lang="ts">
	import EmailPreview from "$lib/components/EmailPreview.svelte";
	import { Button } from "@/components/ui/button";
	import { emails, fetchEmails, selected_previews } from "@/stores/emails";
	import type { ActionData, PageData } from './$types';

    export let data: PageData;

    $: filteredEmails = $emails.filter(email => {
        switch (data.slug) {
            case 'inbox':
                return true;
            case 'drafts':
                return email.flags.draft;
            case 'trash':
                return email.flags.deleted;
            default:
                return false;
        }
    });

    fetchEmails();
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
        {#each filteredEmails as email, id}
            <EmailPreview {id} {email} />
        {/each}
    </div>
</div>