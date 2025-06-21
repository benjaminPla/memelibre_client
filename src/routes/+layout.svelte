<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import { onMount } from 'svelte';

	let notificationText: string = '';
	let notificationTimeout: NodeJS.Timeout;
	let notificationType: 'info' | 'error' = 'info';
	let notificationVisible: boolean = false;

	const showNotification = (type: 'info' | 'error' = 'info', message: string = ''): void => {
		if (notificationTimeout) {
			clearTimeout(notificationTimeout);
		}

		if (!message) {
			notificationVisible = false;
			return;
		}

		notificationVisible = true;
		notificationType = type;
		notificationText = type === 'error' ? 'Error: ' + message : 'Info: ' + message;

		notificationTimeout = setTimeout(() => {
			notificationText = '';
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
	class="notification-container {notificationType}"
	style="display: {notificationVisible ? 'flex' : 'none'}"
>
	<p>{notificationText}</p>
	<button id="notification-btn" on:click={hideNotification}>X</button>
</div>

<Navbar />

<slot />
