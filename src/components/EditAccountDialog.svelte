<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Form from '$lib/components/ui/form';
	import * as Avatar from '$lib/components/ui/avatar';
	import { superForm, type Infer, type SuperValidated } from 'sveltekit-superforms';
	import { accountSettingsSchema, type AccountSettingsSchema } from './schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { Input } from '@/components/ui/input';
	import { Button } from '@/components/ui/button';
	import { Description } from 'formsnap';
	import { Label } from '@/components/ui/label';
	import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';
	import { fetchAccounts } from '@/store';
	import { toast } from 'svelte-sonner';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';

	export let account: Data.Account;
	export let form_data: SuperValidated<Infer<AccountSettingsSchema>>;
	let form_changed = false;
	export let open = false;

	function hashStr(str: string) {
		var hash = 0,
			i,
			chr;
		if (str.length === 0) return hash;
		for (i = 0; i < str.length; i++) {
			chr = str.charCodeAt(i);
			hash = (hash << 5) - hash + chr;
			hash |= 0; // Convert to 32bit integer
		}
		return hash;
	}

	function getEmailInitials(email: string) {
		return email
			.split('@')[0]
			.split('.')
			.map((part) => part[0])
			.join('');
	}

	function parseAccountForm(formData: FormData): InvokeArgs | undefined {
		if (account) {
			const normal_fields = {
				id: account.id,
				email: formData.get('email') as string,
				imap_host: formData.get('imap_host') as string,
				imap_port: parseInt(formData.get('imap_port') as string),
				username: formData.get('username') as string,
				display_name: formData.get('display_name') as string
			};
			const password = formData.get('password') as string;
			if (password === '') {
				return {
					account: { ...normal_fields, password: account.password }
				};
			}
			return {
				account: { ...normal_fields, password: password }
			};
		}
	}

	function isFormDataInitial() {
		return JSON.stringify($formData) === JSON.stringify(initialFormData);
	}

	const form = superForm(form_data, {
		validators: zodClient(accountSettingsSchema),
		id: `account-settings-form-${account.id}`,
		onChange(event) {
			if (!isFormDataInitial()) {
				form_changed = true;
			} else {
				form_changed = false;
			}
		},
		onSubmit(input) {
			console.log('input', parseAccountForm(input.formData));
			invoke('update_account', parseAccountForm(input.formData))
				.then((result) => {
					open = false;
					fetchAccounts();
					invoke('start_specific_imap_thread', { id: account.id, account: account })
						.then(() => {
							console.log('imap thread started');
						})
						.catch((error) => {
							console.log('error', error);
						});
					toast('Account updated successfully');
				})
				.catch((error) => {
					console.log('error', error);
				});
		},
		onError(event) {
			console.log('errors', event);
		}
	});

	const { form: formData, enhance } = form;
	const initialFormData = structuredClone($formData);
	if (account) {
		initialFormData.display_name = account.display_name;
		initialFormData.email = account.email;
		initialFormData.username = account.username;
		initialFormData.imap_host = account.imap_host;
		initialFormData.imap_port = account.imap_port;
		formData.set(initialFormData);
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger><slot /></Dialog.Trigger>
	<Dialog.Content class="w-screen max-w-none lg:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Account settings</Dialog.Title>
			<Dialog.Description>Update your account information</Dialog.Description>
		</Dialog.Header>
		<div class="flex gap-4">
			<div class="w-full">
				<form method="POST" use:enhance id={`account-settings-form-${account.id}`} class="w-full">
					<div class="flex gap-6 lg:block">
						<Form.Field {form} name="display_name" class="flex-1">
							<Form.Control let:attrs>
								<Form.Label>Display Name</Form.Label>
								<Input {...attrs} bind:value={$formData.display_name} type="text" />
							</Form.Control>
							<!-- <Form.Description>This is your public display name.</Form.Description> -->
							<Form.FieldErrors />
						</Form.Field>
						<div class="flex flex-1">
							<Form.Field {form} name="email" class="grow">
								<Form.Control let:attrs>
									<Form.Label>Email</Form.Label>
									<Input {...attrs} bind:value={$formData.email} type="email" />
								</Form.Control>
								<!-- <Form.Description>This is your public display name.</Form.Description> -->
								<Form.FieldErrors />
							</Form.Field>
							<div class="px-6">
								<Label>Avatar</Label>
								<Avatar.Root class="mt-2">
									<Avatar.Image
										src="https://api.dicebear.com/7.x/shapes/svg?seed={hashStr($formData.email)}"
										alt="Profile picture"
									/>
									<Avatar.Fallback>{getEmailInitials(account.email)}</Avatar.Fallback>
								</Avatar.Root>
							</div>
						</div>
					</div>

					<h1 class="mt-6 text-lg font-semibold leading-none tracking-tight">Login Information</h1>
					<p class="mb-2 pt-1.5 text-sm text-muted-foreground">Manage your login information</p>
					<div class="flex gap-6 lg:block">
						<Form.Field {form} name="username" class="flex-1">
							<Form.Control let:attrs>
								<Form.Label>Username</Form.Label>
								<Input {...attrs} bind:value={$formData.username} />
							</Form.Control>
							<!-- <Form.Description>This is your public display name.</Form.Description> -->
							<Form.FieldErrors />
						</Form.Field>
						<Form.Field {form} name="password" class="flex-1">
							<Form.Control let:attrs>
								<Form.Label>Password</Form.Label>
								<Input {...attrs} bind:value={$formData.password} type="password" />
							</Form.Control>
							<!-- <Form.Description>This is your public display name.</Form.Description> -->
							<Form.FieldErrors />
						</Form.Field>
					</div>

					<h1 class="mt-6 text-lg font-semibold leading-none tracking-tight">Imap Information</h1>
					<p class="mb-2 pt-1.5 text-sm text-muted-foreground">Set your imap information</p>
					<div class="flex gap-6 lg:block">
						<div>
							<Form.Field {form} name="imap_host" class="flex-1">
								<Form.Control let:attrs>
									<Form.Label>Imap Host</Form.Label>
									<Input {...attrs} bind:value={$formData.imap_host} />
								</Form.Control>
								<!-- <Form.Description>This is your public display name.</Form.Description> -->
								<Form.FieldErrors />
							</Form.Field>
						</div>
						<div>
							<Form.Field {form} name="imap_port" class="flex-1">
								<Form.Control let:attrs>
									<Form.Label>Imap Port</Form.Label>
									<Input {...attrs} bind:value={$formData.imap_port} />
								</Form.Control>
								<!-- <Form.Description>This is your public display name.</Form.Description> -->
								<Form.FieldErrors />
							</Form.Field>
						</div>
					</div>
				</form>
			</div>
		</div>
		<Dialog.Footer>
			<AlertDialog.Root>
				<AlertDialog.Trigger asChild let:builder>
					<Button builders={[builder]} variant="destructive">Delete Account</Button>
				</AlertDialog.Trigger>
				<AlertDialog.Content>
					<AlertDialog.Header>
						<AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
						<AlertDialog.Description>
							This action cannot be undone. This will permanently delete your account and remove
							your data.
						</AlertDialog.Description>
					</AlertDialog.Header>
					<AlertDialog.Footer>
						<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
						<AlertDialog.Action
							on:click={() => {
								invoke('delete_account', { id: account.id })
									.then((result) => {
										open = false;
										fetchAccounts();
										toast('Account deleted successfully');
									})
									.catch((error) => {
										console.log('error', error);
									});
							}}>Continue</AlertDialog.Action
						>
					</AlertDialog.Footer>
				</AlertDialog.Content>
			</AlertDialog.Root>
			<Button type="submit" form={`account-settings-form-${account.id}`} disabled={!form_changed}
				>Save changes</Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
