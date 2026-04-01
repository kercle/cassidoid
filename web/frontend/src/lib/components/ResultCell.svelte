<script lang="ts">
	import type { CassidaResult } from '$lib/cassida/types/CassidaResult';
	import HelpBuiltin from './help/HelpBuiltin.svelte';
	import HelpTableOfContents from './help/HelpTableOfContents.svelte';
	import Math from './Math.svelte';
	import Plot from './Plot.svelte';

	export let index: number = 1;
	export let entry: CassidaResult | undefined = undefined;

	const getRawInput = (entry: CassidaResult | undefined) => {
		if (!entry) {
			return null;
		}

		if ('input' in entry?.content) {
			return entry?.content.input;
		} else {
			return null;
		}
	};
</script>

<div class="group w-full">
	<div class="bg-base-200 relative">
		{#if entry?.type == 'err' && getRawInput(entry)}
			<div class="overflow-x-auto bg-red-200 py-2 pl-6">
				<p>{getRawInput(entry)}</p>
			</div>
		{:else}
			<div class="flex flex-row">
				<div class="bg-base-200 text-info-content flex w-20 items-center justify-center">
					(%i{index})
				</div>
				<div class="bg-base-200 overflow-x-auto pt-3 pb-2 pl-6">
					{getRawInput(entry)}
				</div>
			</div>
		{/if}
	</div>

	{#if entry?.type == 'err' && entry.content.type == 'parseError'}
		<div class=" overflow-x-auto border border-red-200 bg-white py-2 pl-6">
			<b class="mr-2">Error:</b>{entry.content.msg}
		</div>
	{:else if entry?.type == 'ok' && entry.content.type == 'expression'}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Math expr={entry.content.output.latex} />
			</div>
		</div>
	{:else if entry?.type == 'ok' && entry.content.type == 'plot'}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Plot data={entry.content.data} />
			</div>
		</div>
	{:else if entry?.type == 'ok' && entry.content.type == 'helpTableOfContents'}
		<HelpTableOfContents builtins={entry.content.builtins} />
	{:else if entry?.type == 'ok' && entry.content.type == 'helpBuiltin'}
		<HelpBuiltin
			title={entry.content.title}
			patterns={entry.content.patterns}
			examples={entry.content.examples}
			related={entry.content.related}
		/>
	{:else}
		<p>Unknown server message.</p>
	{/if}
</div>
