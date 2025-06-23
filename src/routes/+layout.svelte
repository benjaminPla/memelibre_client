<script lang="ts">
	import Close from '$lib/components/icons/Close.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	type NotificationType = 'error' | 'info' | 'success';

	let notificationText: string = '';
	let notificationTimeout: NodeJS.Timeout;
	let notificationType: NotificationType;
	let notificationVisible: boolean = false;

	const showNotification = (type: NotificationType = 'info', message: string = ''): void => {
		if (notificationTimeout) {
			clearTimeout(notificationTimeout);
		}

		if (!message) {
			notificationVisible = false;
			return;
		}

		notificationVisible = true;
		notificationType = type;
		notificationText =
			type === 'error' ? 'Error: ' + message : type === 'success' ? message : 'Info: ' + message;

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
	<button id="notification-btn" on:click={hideNotification}><Close /></button>
</div>

<Navbar isLoggedIn={data.isLoggedIn} user={data.user} />

<slot />
