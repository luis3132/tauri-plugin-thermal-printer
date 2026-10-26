<script>
  import { app, buildRequest, printSections } from '../lib/state.svelte.js'

  let source = $state(
    JSON.stringify(
      [
        { GlobalStyles: { reset: true } },
        { Title: { text: 'JSON libre' } },
        { Text: { text: 'Edita estas secciones y pulsa Imprimir.', styles: { align: 'center' } } },
        { Line: { character: '-' } },
        { Qr: { data: 'https://tauri.app', size: 5, error_correction: 'M', model: 2, align: 'center' } },
        { Feed: { feed_type: 'lines', value: 3 } },
        { Cut: { mode: 'partial', feed: 4 } },
      ],
      null,
      2,
    ),
  )

  const parsed = $derived.by(() => {
    try {
      const value = JSON.parse(source)
      return Array.isArray(value) ? { value } : { error: 'Debe ser un arreglo de secciones' }
    } catch (e) {
      return { error: e.message }
    }
  })
</script>

<p class="intro">
  Escribe las <code>sections</code> en JSON tal cual las recibe el plugin. Útil para reproducir un bug con el payload
  exacto de tu app.
</p>

<textarea class="editor" spellcheck="false" bind:value={source}></textarea>

{#if parsed.error}
  <p class="error">JSON inválido: {parsed.error}</p>
{/if}

<div class="actions">
  <button class="primary" disabled={app.busy || !!parsed.error} onclick={() => printSections('JSON libre', parsed.value)}
    >Imprimir</button
  >
</div>

<details>
  <summary>Petición completa que se enviará</summary>
  <pre>{JSON.stringify(buildRequest(parsed.value ?? []), null, 2)}</pre>
</details>

<style>
  .intro {
    color: var(--muted);
    margin-top: 0;
  }
  .editor {
    width: 100%;
    min-height: 320px;
    font-family: ui-monospace, 'JetBrains Mono', monospace;
    font-size: 0.82rem;
    box-sizing: border-box;
  }
  .error {
    color: var(--error);
  }
  .actions {
    margin: 0.75rem 0;
  }
  pre {
    font-size: 0.75rem;
    overflow: auto;
    background: var(--surface);
    padding: 0.75rem;
    border-radius: 8px;
  }
</style>
