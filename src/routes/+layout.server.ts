import { env } from '$env/dynamic/public';
import { error, redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { User } from '$lib/types/index';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const sessionToken: string | undefined = cookies.get('session_token');
	const isLoggedIn = !!sessionToken;

	if (!sessionToken) {
		return { isLoggedIn, user: null };
	}

	const apiUrl = env.PUBLIC_API_URL;

	try {
		const response = await fetch(`${apiUrl}/auth/me`, {
			headers: {
				Cookie: `session_token=${sessionToken}`
			}
		});

		if (!response.ok) {
			redirect(302, `${apiUrl}/auth/login`);
		}

		const user: User = await response.json();

		return { isLoggedIn, user };
	} catch (e: any) {
		if (e?.status) throw e;

		console.error(`src/routes/+layout.server.ts - Error 500 - ${e}`);
		error(500);
	}
};
