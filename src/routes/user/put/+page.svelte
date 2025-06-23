<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { invalidateAll } from '$app/navigation';
	export let data: PageData;

	const apiUrl = env.PUBLIC_API_URL;
	let username = data.user?.username || '';
	let updating = false;

	async function handleSubmit(event: Event) {
		event.preventDefault();
		updating = true;
		window.showNotification?.('info', 'Actualizando...');

		try {
			const response = await fetch(`${apiUrl}/user/put`, {
				body: JSON.stringify({ username }),
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				method: 'PUT'
			});

			if (!response.ok) {
				switch (response.status) {
					case 409: {
						window.showNotification?.('error', 'Ese nombre de usuario ya está en uso');
						break;
					}
					default: {
						const errorMessage = await response.text();
						window.showNotification?.('error', errorMessage || response.statusText);
						break;
					}
				}
				return;
			}

			window.showNotification?.('success', '¡Éxtasis!');
            invalidateAll()
		} catch {
			window.showNotification('error', 'Qué pasó ahora, la puta madre');
		} finally {
			updating = false;
		}
	}
</script>

<form on:submit={handleSubmit}>
	<div>
		<label for="username">Nombre de usuario:</label>
		<input
			type="text"
			id="username"
			name="username"
			bind:value={username}
			required
			maxlength="32"
		/>
	</div>
	<button type="submit" disabled={updating || !username.trim()}>
		{updating ? 'ACTUALIZANDO...' : 'ACTUALIZAR'}
	</button>
</form>
