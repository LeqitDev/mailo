import { mailAccountSchema } from "./schema";
import { superValidate } from "sveltekit-superforms/server";

export const load = async () => {
    const form = await superValidate(mailAccountSchema);
    console.log('form', form);
    return {
        form
    }
}