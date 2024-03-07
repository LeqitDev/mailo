<script lang="ts">
	import { mailAccountSchema, type MailAccount } from "./schema";
    import * as Form from "$lib/components/ui/form";
	import { invoke } from "@tauri-apps/api/tauri";
	import { parseAccountForm } from "@/utils";
	import { fetchAccounts } from "@/stores/accounts";
    import { toast } from "svelte-sonner";
	import { type SuperValidated, type Infer, superForm } from "sveltekit-superforms";
    import { zodClient } from "sveltekit-superforms/adapters";
	import { Input } from "@/components/ui/input";

    export let id = "mail-account-form";
    export let form_data: SuperValidated<Infer<MailAccount>>;
    export let onSuccessfulSubmit: () => void = () => {};

    /* const options: FormOptions<MailAccount> = {
        validators: mailAccountSchema,
        applyAction: false,
        onSubmit: (values) => {
            var form = new FormData(values.formElement);
            
            invoke('add_account', parseAccountForm(form)).then((result) => {
                onSuccessfulSubmit();
                fetchAccounts();
                toast('Account added successfully');
            }).catch((error) => {
                console.log('error', error);
            });
        },

        onError: (errors) => {
            console.log('errors', errors);
        }
    } */

    const form = superForm(form_data, {
        validators: zodClient(mailAccountSchema),
        onSubmit(input) {
            invoke('add_account', parseAccountForm(input.formData)).then((result) => {
                onSuccessfulSubmit();
                fetchAccounts();
                toast('Account added successfully');
            }).catch((error) => {
                console.log('error', error);
            });
        },
        onError(event) {
            console.log('errors', event);
        },
    });

    const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance {id}>
    <Form.Field {form} name="email">
        <Form.Control let:attrs>
            <Form.Label>Email</Form.Label>
            <Input {...attrs} bind:value={$formData.email} type="email" />
        </Form.Control>
        <!-- <Form.Description>This is your public display name.</Form.Description> -->
        <Form.FieldErrors />
    </Form.Field>
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
</form>