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
	import { Button } from '@/components/ui/button';
	import { Switch } from '@/components/ui/switch';
    import * as Select from '@/components/ui/select';
	import MinimalThemeSwitcher from '§/components/MinimalThemeSwitcher.svelte';

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

    function capitalizeFirstLetter(string: string) {
        return string.charAt(0).toUpperCase() + string.slice(1);
    }
</script>

<SettingsLayout bind:subsite>
	<div class="grid h-full w-full gap-8 overflow-auto p-2">
		<InView bind:isInViewProp={generalInView}>
			<div class="grid gap-6">
				<div>
					<p id="general" class="text-2xl font-semibold">General</p>
					<hr class="mt-1" />
				</div>
				<div class="grid gap-4">
					<div class="grid gap-1">
                        <div class="flex gap-4 items-center">
                            <p class="text-xl font-semibold">Lazy Loading</p>
                            <Switch bind:checked={$settings.lazyLoading.enabled} />
                        </div>
						<p class="text-sm text-muted-foreground">
							If enabled, the application will only load a certain amount of emails at a time. This
							can improve performance on slower devices.
						</p>
					</div>
					<div class="grid gap-2 pl-2 border-l">
						<div class="grid gap-1">
							<Label for="lazyLoadingAmount"
								>Amount of mails to load with lazy loading enabled</Label
							>
							<Input
								id="lazyLoadingAmount"
								class="max-w-xs"
								bind:value={$settings.lazyLoading.amount}
								disabled={!$settings.lazyLoading.enabled}
							/>
						</div>
					</div>
				</div>
			</div>
		</InView>
		<InView bind:isInViewProp={accountInView}>
			<div class="grid gap-4">
				<div>
					<p id="accounts" class="text-2xl font-semibold">Accounts</p>
					<hr class="mt-1" />
				</div>
				<div>
					<SettingsAccountViewer {data} />
				</div>
			</div>
		</InView>
		<InView bind:isInViewProp={appearanceInView}>
			<div class="grid gap-6">
				<div>
					<p id="appearance" class="text-2xl font-semibold">Appearance</p>
					<hr class="mt-1" />
				</div>
				<MinimalThemeSwitcher size="lg" title="Theme" />
                <div class="grid gap-4">
                    <p class="text-xl font-semibold">Font</p>
                    <Select.Root selected={{label: capitalizeFirstLetter($settings.fontFamily), value: $settings.fontFamily}} onSelectedChange={value => {value && ($settings.fontFamily = value.value)}}>
                        <Select.Trigger class="max-w-sm">
                            <Select.Value placeholder="Select font" />
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Item value="sans">Sans</Select.Item>
                            <Select.Item value="serif">Serif</Select.Item>
                            <Select.Item value="mono">Mono</Select.Item>
                        </Select.Content>
                    </Select.Root>
                </div>
				<div class="grid gap-4">
					<div class="grid gap-1">
						<p class="text-xl font-semibold">Dashboard email view count</p>
						<p class="text-sm text-muted-foreground">
							The amount of new or unseen emails to show in the dashboard view.
						</p>
					</div>
					<div class="grid gap-2">
						<Input class="max-w-xs" bind:value={$settings.dashboardViewCount} />
					</div>
				</div>
			</div>
		</InView>
		<div class="grid gap-6">
            <div>
                <p id="security" class="text-2xl font-semibold">Security</p>
                <hr class="mt-1" />
            </div>
			<div class="grid gap-4">
				<div class="grid gap-1">
                    <div class="flex gap-4 items-center">
                        <p class="text-xl font-semibold">Master password</p>
                        <Switch bind:checked={$backendSettings.masterpassword} />
                    </div>
					<p class="text-sm text-muted-foreground">
						If enabled, you will have to enter your masterpassword on every new session, to access
						your account and load or update your emails.
					</p>
				</div>
                <div class="grid gap-2 pl-2 border-l">
                    <div class="grid gap-1">
                        <p>Set sessions master password</p>
                        <Button class="w-max" disabled={!$backendSettings.masterpassword}>Set password</Button>
                    </div>
                    <div class="grid gap-1">
                        <p>Change master password</p>
                        <Button class="w-max" disabled={!$backendSettings.masterpassword}>Change password</Button>
                    </div>
                </div>
			</div>
		</div>
		<div class="h-96 w-full overflow-auto p-2">
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
