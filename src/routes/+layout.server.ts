import { env } from '$env/dynamic/public';
import { error } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { User } from '$lib/types/index';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	const sessionToken: string | undefined = cookies.get('session_token');

	if (!sessionToken) {
		return { isLoggedIn: false, user: null };
	}

	const apiUrl = env.PUBLIC_API_URL;

	try {
		const response = await fetch(`${apiUrl}/auth/me`, {
			headers: {
				Cookie: `session_token=${sessionToken}`
			}
		});

		if (!response.ok) {
			cookies.set('session_token', '', {
				path: '/',
				httpOnly: true,
				sameSite: 'lax',
				expires: new Date(0)
			});
			return { isLoggedIn: false, user: null };
		}

		const user: User = await response.json();

		return { isLoggedIn: true, user };
	} catch (e: any) {
		if (e?.status) throw e;

		console.error(`src/routes/+layout.server.ts - Error 500 - ${e}`);
		error(500);
	}
};
