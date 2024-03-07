import { accountSettingsSchema, mailAccountSchema } from "$lib/custom/components/schema";
import { zod } from "sveltekit-superforms/adapters";
import { superValidate } from "sveltekit-superforms";

export const load = async () => {
    const add_account_form = await superValidate(zod(mailAccountSchema));
    const account_settings_form = await superValidate(zod(accountSettingsSchema));
    return {
        add_account_form,
        account_settings_form
    }
}