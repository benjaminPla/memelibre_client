declare global {
	namespace App {
		interface Window {
			showNotification?: (type?: 'info' | 'error', message?: string) => void;
		}
	}
}

declare module '$env/dynamic/public' {
	export const PUBLIC_API_URL: string;
}

export {};
