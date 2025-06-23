<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Like from '$lib/components/icons/Like.svelte';
	import Share from '$lib/components/icons/Share.svelte';

	export let meme: { id: string; image_url: string; like_count: number };

	const apiUrl = env.PUBLIC_API_URL;
	let isLiking = false;
	let currentLikeCount = meme.like_count;

	async function handleLike() {
		if (isLiking) return;
		isLiking = true;

		try {
			const response = await fetch(`${apiUrl}/like/${meme.id}`, {
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
		<button on:click={handleShare}><Share /></button>
	</div>
</div>
