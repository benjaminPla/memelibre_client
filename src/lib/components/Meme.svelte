<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Save from '$lib/components/icons/Save.svelte';
	import Like from '$lib/components/icons/Like.svelte';
	import Share from '$lib/components/icons/Share.svelte';

	export let meme: { id: string; image_url: string; like_count: number };
	export let unsaveMeme: (id: string) => void;

	const apiUrl = env.PUBLIC_API_URL;
	let currentLikeCount = meme.like_count;
	let isLiking = false;
	let isSaving = false;

	async function handleLike() {
		if (isLiking) return;
		isLiking = true;

		try {
			const response = await fetch(`${apiUrl}/like/post/${meme.id}`, {
				credentials: 'include',
				method: 'POST'
			});
			if (!response.ok) {
				if (response.status === 401) {
					window.location = `${apiUrl}/auth/login`;
				}
			}
			if (response.status === 201) {
				currentLikeCount++;
			} else if (response.status === 204) {
				currentLikeCount--;
			}
		} catch {
			window.showNotification('error', 'Qué pasó ahora, la puta madre');
		} finally {
			isLiking = false;
		}
	}

	async function handleSave() {
		if (isSaving) return;
		isSaving = true;

		try {
			const response = await fetch(`${apiUrl}/save/post/${meme.id}`, {
				credentials: 'include',
				method: 'POST'
			});
			if (!response.ok) {
				if (response.status === 401) {
					window.location = `${apiUrl}/auth/login`;
				}
			}
			if (response.status === 201) {
				window.showNotification('success', 'Ahora este meme es propiedad privada');
			} else if (response.status === 204) {
				unsaveMeme?.(meme.id);
			}
		} catch {
			window.showNotification('error', 'Qué pasó ahora, la puta madre');
		} finally {
			isSaving = false;
		}
	}

	function handleShare() {
		const shareUrl = `${window.location.origin}/meme/${meme.id}`;

		if (navigator.share) {
			try {
				navigator.share({
					title: document.title,
					text: '¡Viva la libertad, carajo!',
					url: shareUrl
				});
			} catch {
				window.showNotification?.('error', 'Link copiado!');
			}
		} else {
			try {
				navigator.clipboard.writeText(shareUrl);
				window.showNotification?.('info', 'Link copiado!');
			} catch {
				window.showNotification?.('error', 'Link no copiado');
			}
		}
	}
</script>

<div class="meme-container">
	<img class="meme" src={meme.image_url} alt="memelibre" loading="lazy" />
	{#if meme.username}
		<p class="username">{meme.username}</p>
	{/if}
	<div class="actions-container">
		<button on:click={handleLike}><Like />{currentLikeCount}</button>
		<button on:click={handleSave}><Save /></button>
		<button on:click={handleShare}><Share /></button>
	</div>
</div>
