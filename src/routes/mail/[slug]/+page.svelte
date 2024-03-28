<script lang="ts">
	import EmailPreview from "§/components/EmailPreview.svelte";
	import { Button } from "@/components/ui/button";
	import { emailSortDateFunction, emails, fetchEmails, selected_previews } from '@/store';
	import type { PageData } from './$types';
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";
	import { settings, searchEmail, search_string } from '@/store';
	import { selected_account } from '@/store';
    import * as Resizable from '@/components/ui/resizable';
	import type { PaneAPI } from "paneforge";
	import { ArrowLeftIcon } from "lucide-svelte";

    export let data: PageData;
	$: ready = false;
    $: visible_emails = $settings.lazyLoading.enabled ? $settings.lazyLoading.amount : 99999;
    let email_view: HTMLDivElement;
    let viewPane: PaneAPI;
    let previewEmail: Data.Email | undefined;
    let previewFrame: HTMLIFrameElement;

	onMount(() => {
		ready = true;
	});

    function addStylesToIframe() {
        /* let arrStyleSheets = document.styleSheets;
        console.log(arrStyleSheets);
        // add stylesheets to iframe
        for (let i = 0; i < arrStyleSheets.length; i++) {
            let styleSheet = arrStyleSheets[i] as CSSStyleSheet;
            if (styleSheet.ownerNode) {
                console.log(previewFrame.src);
                // get head
                previewFrame.getElementsByTagName('head')[0].appendChild(styleSheet.ownerNode.cloneNode(true));
                // previewFrame.contentDocument?.head.appendChild(styleSheet.ownerNode.cloneNode(true));
            }
        } */
    }

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

    function onEntryClicked() {
        previewEmail = filteredEmails[$selected_previews[$selected_previews.length - 1]]; 
        addStylesToIframe();
        if (viewPane.isCollapsed()) {
            viewPane.expand();
        }
    }

    function parseDate(email: Data.Email): Date {
        const [datePart, timePart] = email.date.split(' ');
        const [year, month, day] = datePart.split('-').map(Number);
        const [hours, minutes, seconds] = timePart.split(':').map(Number);
        return new Date(year, month - 1, day, hours, minutes, seconds)
    }

    function parseEMailBlob(email: Data.Email | undefined): string {
        if (!email) return '';
        let body = email.body.replaceAll("<a", "<a class='underline text-muted-foreground'").replaceAll(/<style>(.|\r\n|\n\r|\n)*<\/style>/g, '').replaceAll(/style=".*?"/g, '');
        // replace https://.../ with <a href="https://.../">https://.../</a> also check that there are possible <br> tags in the link
        // body = body.replaceAll(/(https?:\/\/[^\s<]+)(?![^<]*>)/g, '<a class="underline text-muted-foreground" href="$1">$1</a>');
        let date = parseDate(email).toLocaleString();
        let arrStyleSheets = document.styleSheets;
        let styles: string = "";
        for (let i = 0; i < arrStyleSheets.length; i++) {
            let styleSheet = arrStyleSheets[i] as CSSStyleSheet;
            if (styleSheet.ownerNode) {
                if (styleSheet.ownerNode instanceof HTMLElement) {
                    styles += styleSheet.ownerNode.outerHTML + "\n";
                }
            }
        }
        let html = `\ufeff<html class="dark"><head>${styles}</head>
            <body><div class="h-full flex flex-col">
            <div class="w-full border-b flex justify-between items-center px-2 py-1 gap-5 h-20">
                <div class="flex items-center gap-2">
                    <div>
                        <h2 class="text font-semibold capitalize line-clamp-2">${email.subject}</h2>
                        <p class="text-xs">Von: ${email.sender}</p>
                    </div>
                </div>
                <div class="flex gap-2 items-center text-sm">
                    <p>${date}</p>
                </div>
            </div>
            <div class="flex-1 overflow-y-auto">
                <div class="p-2">
                    ${body}
                </div>
            </div>
        </div></body></html>`;
        const blob = new Blob([html], { type: 'text/html' });
        return URL.createObjectURL(blob);
    }

    /* search_string.subscribe((value) => {
        if (email_view) {
            visible_emails = 5000;
        }
    }); */

    // fetchEmails();
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
        <Resizable.PaneGroup direction="horizontal">
            <Resizable.Pane>
                <div class="flex-1 overflow-y-auto h-full" bind:this={email_view} on:scroll={onScroll}>
                    {#each filteredEmails.slice(0, visible_emails) as email, id (email.id)}
                        <EmailPreview {id} {email} onClick={onEntryClicked} />
                    {/each}
                </div>
            </Resizable.Pane>
            <Resizable.Handle />
            <Resizable.Pane maxSize={45} minSize={25} defaultSize={0} collapsible={true} bind:pane={viewPane}>
                <!-- {#if previewEmail} -->
                    <iframe bind:this={previewFrame} class="h-full w-full overflow-y-auto bg-background dark:bg-foreground" title="email" src={parseEMailBlob(previewEmail)} sandbox="allow-scripts">
                    </iframe>
                <!-- {/if} -->
            </Resizable.Pane>
        </Resizable.PaneGroup>
    </div>
{/if}