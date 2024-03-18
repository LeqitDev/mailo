import { accountSettingsSchema } from "§/components/schema";
import { invoke } from "@tauri-apps/api/tauri";
import { zod } from "sveltekit-superforms/adapters";
import { superValidate } from "sveltekit-superforms";

export const load = async ({ params }) => { 
    const result = await invoke("get_account", { id: Number(params.slug) }) as Data.Account;
    result.password = "";
    const form = await superValidate(result, zod(accountSettingsSchema));
    return {
        slug: params.slug,
        form
    }
}