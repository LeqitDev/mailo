<script lang="ts">
	import { events, settings } from '@/store';
	import ThemeSwitcher from '§/components/ThemeSwitcher.svelte';
	import type { PageData } from './$types';
	import AccountViewer from '§/components/AccountViewer.svelte';
	import SettingsLayout from '../../components/SettingsLayout.svelte';
	import InView from '§/components/InView.svelte';
	import { onMount } from 'svelte';
	import SettingsAccountViewer from '§/components/SettingsAccountViewer.svelte';
	import { Input } from '@/components/ui/input';
	import { Label } from '@/components/ui/label';

    export let data: PageData;

    function getTimeString(time: string | undefined) {
        if (!time) return '';
        return new Date(parseInt(time)).toLocaleString();
    }

    function getLogPayload(event_payload: any) {
        return event_payload as Data.LogPayload;
    }

    console.log('events', $events);

    $: generalInView = false;
    $: accountInView = false;
    $: appearanceInView = false;
    $: subsite = getSubsite(generalInView, accountInView, appearanceInView);

    function getSubsite(general: boolean, account: boolean, appearance: boolean): string {
        if (general) return "general";
        if (account) return "accounts";
        if (appearance) return "appearance";
        return "security";
    }
</script>

<SettingsLayout bind:subsite={subsite}>
    <div class="p-2 w-full h-full overflow-auto">
        <InView bind:isInViewProp={generalInView}>
            <p class="text-2xl font-semibold">General</p>
            <hr class="mb-4 mt-1">
        </InView>
        <InView bind:isInViewProp={accountInView}>
            <p class="text-2xl font-semibold mt-8">Accounts</p>
            <hr class="mb-4 mt-1">
            <SettingsAccountViewer {data} />
        </InView>
        <InView bind:isInViewProp={appearanceInView}>
            <p class="text-2xl font-semibold mt-8">Appearance</p>
            <hr class="mb-4 mt-1">
            <div class="pl-4">
                <ThemeSwitcher size="lg" title="Theme" />

                <p class="text-xl font-semibold mt-4 mb-2">Dashboard view count</p>
                <Input type="number" class="max-w-xs" bind:value={$settings.dashboardViewCount} />

                <p class="text-xl font-semibold mt-4 mb-2">Lazy Loading E-Mails</p>
                <Label for="lazyLoadingEnabled">Enable lazy loading</Label>
                <Input id="lazyLoadingEnabled" type="checkbox" checked={$settings.lazyLoading.enabled} on:click={(e) => {$settings.lazyLoading.enabled = !$settings.lazyLoading.enabled}} />
                <Label for="lazyLoadingAmount">Amount of mails to load with lazy loading enabled</Label>
                <Input id="lazyLoadingAmount" type="number" class="max-w-xs" bind:value={$settings.lazyLoading.amount} disabled={!$settings.lazyLoading.enabled} />
            </div>
        </InView>
        <div class="mt-[50rem]">
            <p>hallo</p>
        </div>
        <div class="w-full h-full overflow-auto p-2">
            {#if $events}
                {#each $events as event}
                    <div class="flex gap-3">
                        <p class="min-w-max">{getTimeString(event.time)}</p>
                        {#if event.event === "log"}
                            <p>{getLogPayload(event.payload).log_type}</p>
                            <p>{getLogPayload(event.payload).message}</p>
                        {/if}
                    </div>
                {/each}
            {/if}
        </div>
    </div>
</SettingsLayout>