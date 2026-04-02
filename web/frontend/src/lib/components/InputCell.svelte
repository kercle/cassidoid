<script lang="ts">
	import { appState, submitExpression } from '$lib';

	let inputValue = $state('');
	let historyScroll: number | null = null;
	let backupValue = '';

	function handleArrowUp() {
		const history = $appState.data.input_history;
		const historyLen = history.length;

		if (historyLen === 0) return;

		if (historyScroll === null) {
			backupValue = inputValue;
			historyScroll = historyLen - 1;
		} else if (historyScroll > 0) {
			historyScroll -= 1;
		}

		inputValue = history[historyScroll];
	}

	function handleArrowDown() {
		const history = $appState.data.input_history;
		const historyLen = history.length;

		if (historyScroll === null) return;

		if (historyScroll < historyLen - 1) {
			historyScroll += 1;
			inputValue = history[historyScroll];
		} else {
			historyScroll = null;
			inputValue = backupValue;
		}
	}

	function handleEnter() {
		if (inputValue.trim()) {
			submitExpression(inputValue);
			inputValue = '';
		}

		historyScroll = null;
		backupValue = '';
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'ArrowUp') {
			event.preventDefault();
			handleArrowUp();
		} else if (event.key === 'ArrowDown') {
			event.preventDefault();
			handleArrowDown();
		} else if (event.key === 'Enter') {
			event.preventDefault();
			handleEnter();
		} else if (historyScroll !== null) {
			historyScroll = null;
		}
	}
</script>

<div class="bg-base-200 w-full p-1">
	{#if $appState.data.last_error}
		<div class="border-b-base-300 mb-2 overflow-x-auto border-b py-2 pl-6">
			<p class="font-bold text-red-600">Error:</p>
			<p class="my-3 ml-4">
				{$appState.data.input_history[$appState.data.input_history.length - 1]}
			</p>
			{#if 'msg' in $appState.data.last_error}
				<p class="font-black">{$appState.data.last_error.msg}</p>
			{/if}
		</div>
	{/if}
	<label
		class="input input-ghost flex w-full items-center gap-2 focus-within:bg-transparent focus-within:outline-none"
	>
		<i class="fa-solid fa-angle-right text-info-content"></i>

		<input
			type="text"
			placeholder="Input expression"
			bind:value={inputValue}
			onkeydown={handleKeydown}
		/>
	</label>
</div>
