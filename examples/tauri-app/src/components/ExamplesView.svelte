<script>
  import { EXAMPLE_GROUPS } from '../lib/examples.js'
  import { SAMPLE_IMAGES, urlToDataUrl } from '../lib/images.js'
  import { app, charsPerLine, pixelsWidth, printSections } from '../lib/state.svelte.js'

  let images = $state(null)
  let openJson = $state('')

  // Las imágenes de muestra se cargan una vez como data URI para usarlas en los ejemplos.
  $effect(() => {
    Promise.all(SAMPLE_IMAGES.map(async (img) => [img.id, await urlToDataUrl(img.url)])).then(
      (entries) => (images = Object.fromEntries(entries)),
    )
  })

  function sectionsFor(example) {
    return example.build({ cpl: charsPerLine(), px: pixelsWidth(), images: images ?? {} })
  }
</script>

{#each EXAMPLE_GROUPS as group}
  <section class="group">
    <h2>{group.name}</h2>
    <div class="grid">
      {#each group.examples as example}
        <article class="card">
          <header>
            <h3>{example.name}</h3>
            {#if example.advanced}<span class="badge">Firmware avanzado</span>{/if}
          </header>
          <p>{example.description}</p>
          <div class="actions">
            <button
              class="primary"
              disabled={app.busy || !images}
              onclick={() => printSections(example.name, sectionsFor(example))}>Imprimir</button
            >
            <button
              class="ghost"
              disabled={!images}
              onclick={() => (openJson = openJson === example.id ? '' : example.id)}
              >{openJson === example.id ? 'Ocultar JSON' : 'Ver JSON'}</button
            >
          </div>
          {#if openJson === example.id && images}
            <pre class="json">{JSON.stringify(sectionsFor(example), (k, v) =>
                typeof v === 'string' && v.startsWith('data:') ? `${v.slice(0, 40)}… (${v.length} chars)` : v,
              2)}</pre>
          {/if}
        </article>
      {/each}
    </div>
  </section>
{/each}

<style>
  .group + .group {
    margin-top: 1.5rem;
  }
  h2 {
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--muted);
    margin: 0 0 0.6rem;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 0.75rem;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 0.9rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 0;
  }
  .card header {
    display: flex;
    gap: 0.5rem;
    align-items: baseline;
    justify-content: space-between;
    flex-wrap: wrap;
  }
  h3 {
    margin: 0;
    font-size: 1rem;
  }
  p {
    margin: 0;
    color: var(--muted);
    font-size: 0.88rem;
    flex: 1;
  }
  .badge {
    font-size: 0.7rem;
    padding: 0.1rem 0.45rem;
    border-radius: 999px;
    background: var(--warn-bg);
    color: var(--warn);
  }
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  .json {
    max-height: 260px;
    overflow: auto;
    font-size: 0.75rem;
    background: var(--bg);
    border-radius: 8px;
    padding: 0.5rem;
    margin: 0;
  }
</style>
