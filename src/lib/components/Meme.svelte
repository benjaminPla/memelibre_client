<script lang="ts">
	import { env } from '$env/dynamic/public';
	import LikeIcon from '$lib/components/LikeIcon.svelte';
	import ShareIcon from '$lib/components/ShareIcon.svelte';

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
					window.location = `${apiUrl}/auth`;
				}
			}

			if (response.status === 201) {
				currentLikeCount++;
				window.showNotification('success', '¡Éxtasis!');
			} else if (response.status === 204) {
				currentLikeCount--;
				window.showNotification('success', '¡AFUERA!');
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
	<div class="actions-container">
		<button on:click={handleLike}><LikeIcon />{currentLikeCount}</button>
		<button on:click={handleShare}><ShareIcon /></button>
	</div>
</div>
