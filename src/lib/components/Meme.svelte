<script lang="ts">
	export let meme: { id: string; image_url: string };

	function sharePage() {
		const shareUrl = `${window.location.origin}/meme/${meme.id}`;

		if (navigator.share) {
			navigator
				.share({
					title: document.title,
					text: '¡Viva la libertad, carajo!',
					url: shareUrl
				})
				.catch(() => {});
		} else {
			navigator.clipboard
				.writeText(shareUrl)
				.then(() => {
					window.showNotification?.('info', 'Link copiado!');
				})
				.catch(() => {
					window.showNotification?.('error', 'Link no copiado');
				});
		}
	}
</script>

<div class="meme-container">
	<img class="meme" src={meme.image_url} alt="memelibre" loading="lazy" />
	<button on:click={sharePage}>Compartir</button>
</div>
