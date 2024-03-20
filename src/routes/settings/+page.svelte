<script lang="ts">
	import { backendSettings, events, settings } from '@/store';
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
		if (general) return 'general';
		if (account) return 'accounts';
		if (appearance) return 'appearance';
		return 'security';
	}
</script>

<SettingsLayout bind:subsite>
	<div class="h-full w-full overflow-auto p-2">
		<InView bind:isInViewProp={generalInView}>
			<p id="general" class="text-2xl font-semibold">General</p>
			<hr class="mb-6 mt-1" />
            <div class="mb-2">
                <p class="text-xl font-semibold">Lazy Loading</p>
                <p class="text-sm text-muted-foreground">
                    If enabled, the application will only load a certain amount of emails at a time. This can
                    improve performance on slower devices.
                </p>
            </div>
			<Label for="lazyLoadingEnabled">Enable lazy loading</Label>
			<Input
				id="lazyLoadingEnabled"
				type="checkbox"
				checked={$settings.lazyLoading.enabled}
				on:click={(e) => {
					$settings.lazyLoading.enabled = !$settings.lazyLoading.enabled;
				}}
			/>
			<Label for="lazyLoadingAmount">Amount of mails to load with lazy loading enabled</Label>
			<Input
				id="lazyLoadingAmount"
				class="max-w-xs"
				bind:value={$settings.lazyLoading.amount}
				disabled={!$settings.lazyLoading.enabled}
			/>
		</InView>
		<InView bind:isInViewProp={accountInView}>
			<p id="accounts" class="mt-12 text-2xl font-semibold">Accounts</p>
			<hr class="mb-6 mt-1" />
			<SettingsAccountViewer {data} />
		</InView>
		<InView bind:isInViewProp={appearanceInView}>
			<p id="appearance" class="mt-12 text-2xl font-semibold">Appearance</p>
			<hr class="mb-6 mt-1" />
			<ThemeSwitcher size="lg" title="Theme" />

            <div class="mt-8 mb-2">
                <p class="text-xl font-semibold">Dashboard email view count</p>
                <p class="text-sm text-muted-foreground">
                    The amount of new or unseen emails to show in the dashboard view.
                </p>
            </div>
			<Input class="max-w-xs" bind:value={$settings.dashboardViewCount} />
		</InView>
		<div>
			<p id="security" class="mt-12 text-2xl font-semibold">Security</p>
			<hr class="mb-6 mt-1" />
			<div class="">
				<div class="mb-2">
					<p class="text-xl font-semibold">Masterpassword</p>
					<p class="text-sm text-muted-foreground">
						If enabled, you will have to enter your masterpassword on every new session, to access your
						account and load or update your emails.
					</p>
				</div>
				<Label for="useMasterpassword">Enable master password</Label>
				<Input
					id="useMasterpassword"
					type="checkbox"
					checked={$backendSettings.masterpassword}
					on:click={(e) => {
						$backendSettings.masterpassword = !$backendSettings.masterpassword;
					}}
				/>
				<Label for="masterpassword">Masterpassword</Label>
				<Input
					id="masterpassword"
					class="max-w-xs"
					type="password"
					bind:value={$backendSettings.masterpassword}
					disabled={!$backendSettings.masterpassword}
				/>
			</div>
		</div>
		<div class="mt-[50rem]">
			<p>hallo</p>
		</div>
		<div class="h-full w-full overflow-auto p-2">
			{#if $events}
				{#each $events as event}
					<div class="flex gap-3">
						<p class="min-w-max">{getTimeString(event.time)}</p>
						{#if event.event === 'log'}
							<p>{getLogPayload(event.payload).log_type}</p>
							<p>{getLogPayload(event.payload).message}</p>
						{/if}
					</div>
				{/each}
			{/if}
		</div>
	</div>
</SettingsLayout>
