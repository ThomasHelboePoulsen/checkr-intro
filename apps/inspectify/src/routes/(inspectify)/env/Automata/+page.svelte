<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import { Io } from '$lib/io.svelte';
  import Network from '$lib/components/NetworkJellyfish.svelte';
  import InputOptions from '$lib/components/InputOptions.svelte';

  const io = new Io('Automata', { regex: 'skip' });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="Automata" code="regex" {io} />
  {/snippet}
  {#snippet outputView({ output, referenceOutput })}
    {console.log(output.dot)}
    <div class="relative">
      <div class="absolute inset-0 grid overflow-auto">
        <Network dot={output.dot || ''} />
      </div>
    </div>
  {/snippet}
</Env>
