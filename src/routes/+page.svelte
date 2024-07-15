<script>
	import { onMount } from 'svelte';
	import { get_timestamp, timestamp_fmt } from '$wasm';
	import { browser } from '$app/environment';
	import { fade, blur, fly, slide, crossfade } from 'svelte/transition';

	BigInt.prototype.toJSON = function () {
		return this.toString();
	};

	let last_record = 0;
	let records = [];
	if (browser) {
		let v = localStorage.getItem('records');
		if (v) {
			let saved_records = [];
			for (let k of JSON.parse(v)) {
				saved_records.push(BigInt(k));
			}
			records = saved_records;
		}
	}

	let tm = get_timestamp();
	onMount(() => {
		const interval = setInterval(() => {
			tm = get_timestamp();
		}, 1000);
	});

	function onKeyDown(e) {
		switch (e.keyCode) {
			case 13: {
				records = [...records, tm];
				localStorage.setItem('records', JSON.stringify(records));
			}
		}
	}
</script>

<svelte:window on:keydown|preventDefault={onKeyDown} />

<div class="flex flex-col w-full h-full justify-center items-center space-y-16">
	{#if records.length > 0}
		{#key records[records.length - 1]}
			<div
				transition:slide
				class="text-4xl h-48 w-2/3 bg-red-100 flex flex-col items-center justify-center"
			>
				<div>LAST HIT</div>
				<div>
					{timestamp_fmt(records[records.length - 1], '%H:%M:%S')}
				</div>
			</div>
		{/key}
	{/if}
	<div
		class="text-blue-700 text-9xl h-96 w-2/3 flex flex-col content-center bg-lime-200 flex flex-col items-center justify-center"
	>
		<div class="text-6xl text-black">
			{timestamp_fmt(tm, '%F')}
		</div>
		<div>
			{timestamp_fmt(tm, '%T')}
		</div>
	</div>
	{#if records.length > 0}
		{#key records[records.length - 1]}
			<div
				transition:slide
				class="text-4xl h-48 w-2/3 bg-sky-400 flex flex-col items-center justify-center"
			>
				<div>NEXT HIT</div>
				<div>
					{timestamp_fmt(records[records.length - 1] + BigInt(2 * 60 * 60), '%H:%M:%S')}
				</div>
			</div>
		{/key}
	{/if}
</div>
