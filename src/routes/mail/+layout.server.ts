import { mailAccountSchema } from "$lib/custom/components/schema";
import { zod } from "sveltekit-superforms/adapters";
import { superValidate } from "sveltekit-superforms/server";

export const load = async () => {
    const form = await superValidate(zod(mailAccountSchema));
    return {
        form
    }
}