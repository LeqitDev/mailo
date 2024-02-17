// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	namespace Data {
		interface Account {
			id: string;
			username: string;
			email: string;
			password: string;
			imapHost: string;
			imapPort: string;
		}
	}
}

export {};
