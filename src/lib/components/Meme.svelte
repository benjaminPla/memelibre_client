<script lang="ts">
	import CommentBubble from '$lib/components/icons/CommentBubble.svelte';
	import { env } from '$env/dynamic/public';
	import Like from '$lib/components/icons/Like.svelte';
	import Save from '$lib/components/icons/Save.svelte';
	import Share from '$lib/components/icons/Share.svelte';
	import type { MemeWithUsernameAndCommentsCount } from '$lib/types';

	export let meme: MemeWithUsername;
	export let unsaveMeme: (id: number) => void;

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
				window.showNotification('error', 'Qué pasó ahora, la puta madre');
			}
		} else {
			try {
				navigator.clipboard.writeText(shareUrl);
				window.showNotification?.('info', 'Link copiado!');
			} catch {
				window.showNotification('error', 'Qué pasó ahora, la puta madre');
			}
		}
	}
</script>

<div class="meme-container">
	<img class="meme" src={meme.image_url} alt="memelibre" loading="lazy" />
	<div class="actions-container">
		<a class="button" href={`/meme/${meme.id}`}><CommentBubble />{meme.comment_count}</a>
		<button on:click={handleLike}><Like />{currentLikeCount}</button>
		<button on:click={handleSave}><Save /></button>
		<button on:click={handleShare}><Share /></button>
	</div>
	<p class="username">{meme.username}</p>
</div>
