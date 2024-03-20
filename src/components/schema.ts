import { z } from 'zod';

export const mailAccountSchema = z.object({
    email: z.string().email().min(1),
    username: z.string().min(1),
    password: z.string().min(1),
    imap_host: z.string().min(1),
    imap_port: z.coerce.number().min(1),
    });

export type MailAccount = typeof mailAccountSchema;

export const accountSettingsSchema = z.object({
    email: z.string().email().min(1),
    username: z.string().min(1),
    password: z.string(),
    imap_host: z.string().min(1),
    imap_port: z.coerce.number().min(1),
    display_name: z.string().min(1),
});

export type AccountSettingsSchema = typeof accountSettingsSchema;