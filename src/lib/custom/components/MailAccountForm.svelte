<script lang="ts">
	import { type SuperValidated } from "sveltekit-superforms";
	import { mailAccountSchema, type MailAccount } from "./schema";
    import * as Form from "$lib/components/ui/form";
	import type { FormOptions } from "formsnap";
	import { invoke } from "@tauri-apps/api/tauri";
	import { parseAccountForm } from "@/utils";
	import { fetchAccounts } from "@/stores/accounts";
    import { toast } from "svelte-sonner";

    export let id = "mail-account-form";
    export let form: SuperValidated<MailAccount>;
    export let onSuccessfulSubmit: () => void = () => {};

    const options: FormOptions<MailAccount> = {
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
    }
</script>

<Form.Root form={form} schema={mailAccountSchema} let:config {id} {options}>
    <Form.Field {config} name="email">
        <Form.Item>
            <Form.Label>Email</Form.Label>
            <Form.Input type="email" />
            <!-- <Form.Description>This is your public display name.</Form.Description> -->
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="username">
        <Form.Item>
            <Form.Label>Username</Form.Label>
            <Form.Input />
            <!-- <Form.Description>This is your public display name.</Form.Description> -->
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="password">
        <Form.Item>
            <Form.Label>Password</Form.Label>
            <Form.Input type="password" />
            <!-- <Form.Description>This is your public display name.</Form.Description> -->
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="imap_host">
        <Form.Item>
            <Form.Label>Imap Host</Form.Label>
            <Form.Input />
            <!-- <Form.Description>This is your public display name.</Form.Description> -->
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="imap_port">
        <Form.Item>
            <Form.Label>Imap Port</Form.Label>
            <Form.Input />
            <!-- <Form.Description>This is your public display name.</Form.Description> -->
            <Form.Validation />
        </Form.Item>
    </Form.Field>
</Form.Root>