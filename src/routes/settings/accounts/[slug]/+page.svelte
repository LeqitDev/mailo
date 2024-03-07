<script lang="ts">
	import CustomLayout from "@/custom/layouts/_customLayout.svelte";
    import type { PageData } from "./$types";
    import { accounts } from "@/stores/accounts";
    import * as Avatar from "@/components/ui/avatar";
	import { superForm } from "sveltekit-superforms/client";
	import { Button } from "@/components/ui/button";
	import { ArrowLeftIcon } from "lucide-svelte";
    import * as Form from "$lib/components/ui/form";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { invoke } from "@tauri-apps/api/tauri";
	import { accountSettingsSchema } from "@/custom/components/schema";
	import { Input } from "@/components/ui/input";
	import { fade, fly, slide } from "svelte/transition";

    export let data: PageData;

    let account = $accounts.find((a) => a.id == data.slug);
    let form_changed = false;

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

    function parseAccountForm() {
        if (account) {
            return {
                email: account.email,
                password: account.password,
                imap_host: account.imap_host,
                imap_port: account.imap_port,
                username: account.username,
            }
        }
    }

    function isFormDataInitial() {
        return JSON.stringify($formData) === JSON.stringify(initialFormData);
    }

    const form = superForm(data.form, {
        validators: zodClient(accountSettingsSchema),
        onChange(event) {
            if (!isFormDataInitial()) {
                form_changed = true;
            } else {
                form_changed = false;
            }
        },
        onSubmit(input) {
            /* invoke('add_account', parseAccountForm(input.formData)).then((result) => {
                onSuccessfulSubmit();
                fetchAccounts();
                toast('Account added successfully');
            }).catch((error) => {
                console.log('error', error);
            }); */
        },
        onError(event) {
            console.log('errors', event);
        },
    });

    const { form: formData, enhance } = form;
    const initialFormData = structuredClone($formData);
</script>

{#if account}
    <div class="w-full h-full flex justify-center items-center">
        <!-- <div class="flex items-center justify-between border-b">
            <Button on:click={() => window.history.back()} variant="ghost" size="icon"><ArrowLeftIcon /></Button>
            <h1 class="text-xl font-semibold">Account settings</h1>
            <span />
        </div> -->
        <div class="max-w-lg w-full border rounded-sm h-max">
            <div class="w-full border-b flex gap-6 items-center">
                <Button on:click={() => window.history.back()} variant="ghost" size="icon"><ArrowLeftIcon /></Button>
                <h1 class="text-2xl font-semibold">Account settings</h1>
                <span />
            </div>
            <div class="p-2 flex gap-4">
                    <Avatar.Root>
                        <Avatar.Image
                        src="https://api.dicebear.com/7.x/shapes/svg?seed={hashStr($formData.email)}"
                        alt="Profile picture"
                        />
                        <Avatar.Fallback>{getEmailInitials(account.email)}</Avatar.Fallback>
                    </Avatar.Root>
                <div class="w-full">
                    <form method="POST" use:enhance id="account-settings-form" class="w-full">
                        <h2 class="text-xl font-semibold">Account Information</h2>
                        <p class="text-muted-foreground text-sm mb-2">Update your account information</p>
                        <Form.Field {form} name="display_name">
                            <Form.Control let:attrs>
                                <Form.Label>Display Name</Form.Label>
                                <Input {...attrs} bind:value={$formData.display_name} type="text" />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <Form.Field {form} name="email">
                            <Form.Control let:attrs>
                                <Form.Label>Email</Form.Label>
                                <Input {...attrs} bind:value={$formData.email} type="email" />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <h1 class="text-xl font-semibold mt-4">Login Information</h1>
                        <p class="text-muted-foreground text-sm mb-2">Manage your login information</p>
                        <Form.Field {form} name="username">
                            <Form.Control let:attrs>
                                <Form.Label>Username</Form.Label>
                                <Input {...attrs} bind:value={$formData.username} />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <Form.Field {form} name="password">
                            <Form.Control let:attrs>
                                <Form.Label>Password</Form.Label>
                                <Input {...attrs} bind:value={$formData.password} type="password" />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <h1 class="text-xl font-semibold mt-4">Imap Information</h1>
                        <p class="text-muted-foreground text-sm mb-2">Set your imap information</p>
                        <Form.Field {form} name="imap_host">
                            <Form.Control let:attrs>
                                <Form.Label>Imap Host</Form.Label>
                                <Input {...attrs} bind:value={$formData.imap_host} />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <Form.Field {form} name="imap_port">
                            <Form.Control let:attrs>
                                <Form.Label>Imap Port</Form.Label>
                                <Input {...attrs} bind:value={$formData.imap_port} />
                            </Form.Control>
                            <!-- <Form.Description>This is your public display name.</Form.Description> -->
                            <Form.FieldErrors />
                        </Form.Field>
                        <Form.Button class="mt-2 w-36" disabled={!form_changed}>Save</Form.Button>
                        <!-- <Button form="account-settings-form" type="submit" class="rounded-3xl" size="lg">Save</Button> -->
                    </form>
                </div>
            </div>
        </div>
    </div>
    <div class="absolute bottom-0 p-2 pb-6 flex justify-center w-full overflow-hidden">
        {#if form_changed}
            <div transition:fly={{y: 100}}>
                <Button form="account-settings-form" type="submit" class="rounded-3xl" size="lg">Save</Button>
            </div>
        {/if}
    </div>
{/if}