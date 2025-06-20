<script lang="ts">
	import { onMount } from 'svelte';
	import Navbar from '$lib/components/Navbar.svelte';

	let notificationContainer: HTMLDivElement;
	let notificationText: string = '';
	let notificationVisible: boolean = false;
	let notificationTimeout: NodeJS.Timeout;

	const showNotification = (type: 'info' | 'error' = 'info', message: string = ''): void => {
		if (notificationTimeout) {
			clearTimeout(notificationTimeout);
		}

		if (!message) {
			notificationVisible = false;
			return;
		}

		notificationVisible = true;

		if (type === 'error') {
			notificationText = 'Error: ' + message;
			notificationContainer.style.backgroundColor = 'var(--orange)';
		} else {
			notificationText = 'Info: ' + message;
			notificationContainer.style.backgroundColor = 'var(--yellow)';
		}

		notificationTimeout = setTimeout(() => {
			notificationVisible = false;
		}, 3000);
	};

	const hideNotification = () => {
		notificationVisible = false;
	};

	onMount(() => {
		window.showNotification = showNotification;
	});
</script>

<div
	bind:this={notificationContainer}
	id="notification-container"
	style="display: {notificationVisible ? 'flex' : 'none'}"
>
	<p id="notification-text">{notificationText}</p>
	<button id="notification-btn" on:click={hideNotification}>X</button>
</div>

<Navbar />

<slot />
