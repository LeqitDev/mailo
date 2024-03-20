<script lang="ts">
	import EmailPreview from "§/components/EmailPreview.svelte";
	import { Button } from "@/components/ui/button";
	import { emailSortDateFunction, emails, fetchEmails, selected_previews } from '@/store';
	import type { PageData } from './$types';
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";
	import { settings, searchEmail, search_string } from '@/store';
	import { selected_account } from '@/store';

    export let data: PageData;
	$: ready = false;
    $: visible_emails = $settings.lazyLoading.enabled ? $settings.lazyLoading.amount : 99999;
    let email_view: HTMLDivElement;

	onMount(() => {
		ready = true;
	});

    $: filteredEmails = $emails.filter(email => {
        if (!searchEmail(email, $search_string)) return false;
        if ($selected_account && email.account_id !== $selected_account.id) return false;
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

    function onScroll(e: Event) {
        const target = e.target as HTMLDivElement;
        // at 3/4 of the scroll height, load more emails
        if (target.scrollTop + target.clientHeight >= target.scrollHeight - 300) {
            visible_emails = Number(visible_emails) + Number($settings.lazyLoading.amount);
            if (visible_emails > filteredEmails.length) {
                visible_emails = filteredEmails.length;
            }
        }
    }

    search_string.subscribe((value) => {
        if (email_view) {
            visible_emails = 5000;
        }
    });

    fetchEmails();
</script>
{#if ready}
    <div class="h-full flex flex-col" transition:fade={{duration: 200}}>
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
        <div class="flex-1 overflow-y-auto" bind:this={email_view} on:scroll={onScroll}>
            {#each filteredEmails.slice(0, visible_emails) as email, id (email.id)}
                <EmailPreview {id} {email} />
            {/each}
        </div>
    </div>
{/if}