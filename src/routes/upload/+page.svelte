<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { goto } from '$app/navigation';

	export let data: PageData;

	const apiUrl = env.PUBLIC_API_URL;
	let files: FileList;
	let uploading = false;

	async function handleSubmit(event: Event) {
		event.preventDefault();

		uploading = true;
		window.showNotification?.('info', 'Verificando si es un meme kuka...');

		const formData = new FormData();
		formData.append('file', files[0]);

		try {
			const response = await fetch(`${apiUrl}/meme/post`, {
				method: 'POST',
				body: formData
			});

			if (!response.ok) {
				switch (response.status) {
					case 413: {
						window.showNotification?.('error', 'El meme es muy grande, metele motosierra');
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

			goto('/');
		} catch {
			window.showNotification?.('error', 'Internal server error');
			goto('/error');
		} finally {
			uploading = false;
		}
	}
</script>

<form on:submit={handleSubmit} enctype="multipart/form-data">
	<div>
		<label for="file">Meme:</label>
		<input type="file" id="file" name="file" accept="image/*" bind:files required />
	</div>
	<p>El archivo no puedo ser mayor a 2MB</p>
	<button type="submit" disabled={uploading}>
		{uploading ? 'SUBIENDO...' : 'SUBIR'}
	</button>
</form>
