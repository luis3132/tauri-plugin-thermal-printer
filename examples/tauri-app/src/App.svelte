<script>
  import ExamplesView from './components/ExamplesView.svelte'
  import ImagesView from './components/ImagesView.svelte'
  import TestView from './components/TestView.svelte'
  import JsonView from './components/JsonView.svelte'
  import {
    app,
    isMobile,
    PAPER_SIZES,
    ENCODINGS,
    printerTarget,
    currentTarget,
    charsPerLine,
    pixelsWidth,
    refreshPrinters,
  } from './lib/state.svelte.js'

  const TABS = [
    { id: 'examples', label: 'Ejemplos', component: ExamplesView },
    { id: 'images', label: 'Imágenes', component: ImagesView },
    { id: 'test', label: 'Prueba integral', component: TestView },
    { id: 'json', label: 'JSON libre', component: JsonView },
  ]
  let tab = $state('examples')
  let showLog = $state(true)
  const Current = $derived(TABS.find((t) => t.id === tab).component)

  $effect(() => {
    refreshPrinters()
  })
</script>

<div class="shell">
  <aside class="panel">
    <div class="brand">
      <span class="logo" aria-hidden="true">🧾</span>
      <div>
        <h1>Thermal Printer</h1>
        <small>Demo de tauri-plugin-thermal-printer</small>
      </div>
    </div>

    <section>
      <div class="section-head">
        <h2>Impresora</h2>
        <button class="ghost small" disabled={app.busy} onclick={refreshPrinters}>
          {app.busy ? '…' : 'Buscar'}
        </button>
      </div>

      {#if app.printers.length === 0}
        <p class="empty">
          No se encontró ninguna impresora.
          {#if isMobile}Empareja la impresora Bluetooth en los ajustes de Android y vuelve a buscar.{/if}
        </p>
      {/if}

      <ul class="printers">
        {#each app.printers as p}
          {@const value = printerTarget(p)}
          <li>
            <label class:active={!app.useManual && app.target === value}>
              <input type="radio" name="printer" {value} bind:group={app.target} onchange={() => (app.useManual = false)} />
              <span class="pname">{p.name}</span>
              <span class="meta">{p.interface_type} · {p.status}</span>
              <code>{p.identifier}</code>
            </label>
          </li>
        {/each}
      </ul>

      <label class="check"><input type="checkbox" bind:checked={app.useManual} /> Destino manual</label>
      {#if app.useManual}
        <input
          type="text"
          placeholder={isMobile ? 'AA:BB:CC:DD:EE:FF o 192.168.1.50:9100' : 'Nombre de la impresora'}
          bind:value={app.manualTarget}
        />
      {/if}
      <p class="hint">
        Se envía <code>printer: "{currentTarget() || '—'}"</code>
        ({isMobile ? 'identifier en móvil' : 'nombre en escritorio'}).
      </p>
    </section>

    <section>
      <h2>Papel y codificación</h2>
      <div class="fields">
        <label
          >Ancho
          <select bind:value={app.paperSize}>
            {#each PAPER_SIZES as size}<option value={size}>{size.slice(2)} mm</option>{/each}
          </select>
        </label>
        <label
          >Página de códigos (ESC t)
          <input type="number" min="0" max="255" bind:value={app.codePage} />
        </label>
        <label class="span"
          >Codificación
          <select bind:value={app.encode}>
            {#each ENCODINGS as enc}<option value={enc}>{enc}</option>{/each}
          </select>
        </label>
        <label class="check span"><input type="checkbox" bind:checked={app.useGbk} /> Fallback GBK</label>
      </div>
      <p class="hint">{charsPerLine()} caracteres por línea · {pixelsWidth()} px de ancho</p>
    </section>
  </aside>

  <main>
    <nav class="tabs">
      {#each TABS as t}
        <button class:active={tab === t.id} onclick={() => (tab = t.id)}>{t.label}</button>
      {/each}
    </nav>
    <div class="content">
      <Current />
    </div>
  </main>

  <footer class="log" class:collapsed={!showLog}>
    <div class="log-head">
      <button class="ghost small" onclick={() => (showLog = !showLog)}>
        {showLog ? '▾' : '▸'} Registro ({app.logs.length})
      </button>
      {#if showLog && app.logs.length}
        <button class="ghost small" onclick={() => (app.logs = [])}>Limpiar</button>
      {/if}
    </div>
    {#if showLog}
      <ol>
        {#each app.logs as entry}
          <li class={entry.level}><time>{entry.time}</time> {entry.message}</li>
        {:else}
          <li class="muted">Sin eventos todavía.</li>
        {/each}
      </ol>
    {/if}
  </footer>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 320px minmax(0, 1fr);
    grid-template-rows: minmax(0, 1fr) auto;
    height: 100vh;
    height: 100dvh;
  }
  .panel {
    grid-row: 1 / 3;
    background: var(--surface);
    border-right: 1px solid var(--border);
    overflow-y: auto;
    padding: 1rem;
    padding-top: max(1rem, env(safe-area-inset-top));
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
  main {
    display: flex;
    flex-direction: column;
    min-height: 0;
    padding-top: env(safe-area-inset-top);
  }
  .content {
    overflow-y: auto;
    padding: 1rem;
    flex: 1;
  }
  .log {
    grid-column: 2;
    border-top: 1px solid var(--border);
    background: var(--surface);
    max-height: 30vh;
    display: flex;
    flex-direction: column;
    padding-bottom: env(safe-area-inset-bottom);
  }

  @media (max-width: 760px) {
    .shell {
      grid-template-columns: 1fr;
      grid-template-rows: auto auto auto;
      height: auto;
    }
    .panel {
      grid-row: auto;
      border-right: none;
      border-bottom: 1px solid var(--border);
      overflow: visible;
    }
    .content {
      overflow: visible;
    }
    .log {
      grid-column: 1;
      max-height: none;
    }
  }

  .brand {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
  .logo {
    font-size: 1.9rem;
  }
  h1 {
    font-size: 1.15rem;
    margin: 0;
  }
  .brand small {
    color: var(--muted);
  }
  h2 {
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    color: var(--muted);
    margin: 0 0 0.5rem;
  }
  .section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .empty,
  .hint {
    color: var(--muted);
    font-size: 0.8rem;
    overflow-wrap: anywhere;
  }
  .printers {
    list-style: none;
    padding: 0;
    margin: 0 0 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
  .printers label {
    display: grid;
    grid-template-columns: auto 1fr;
    column-gap: 0.5rem;
    padding: 0.55rem 0.65rem;
    border: 1px solid var(--border);
    border-radius: 10px;
    cursor: pointer;
  }
  .printers label.active {
    border-color: var(--accent);
    background: var(--accent-bg);
  }
  .printers input {
    grid-row: 1 / 4;
    align-self: center;
  }
  .pname {
    font-weight: 600;
    overflow-wrap: anywhere;
  }
  .meta {
    font-size: 0.78rem;
    color: var(--muted);
  }
  .printers code {
    font-size: 0.72rem;
    color: var(--muted);
    overflow-wrap: anywhere;
  }
  .check {
    display: flex;
    gap: 0.45rem;
    align-items: center;
    font-size: 0.88rem;
  }
  .panel input[type='text'] {
    width: 100%;
    box-sizing: border-box;
    margin-top: 0.4rem;
  }
  .fields {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.6rem;
  }
  .fields label:not(.check) {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.78rem;
    color: var(--muted);
  }
  .fields .span {
    grid-column: 1 / 3;
  }
  .fields input,
  .fields select {
    width: 100%;
    box-sizing: border-box;
  }

  .tabs {
    display: flex;
    gap: 0.25rem;
    padding: 0.6rem 1rem 0;
    border-bottom: 1px solid var(--border);
    overflow-x: auto;
  }
  .tabs button {
    border: none;
    background: none;
    border-bottom: 2px solid transparent;
    border-radius: 0;
    padding: 0.6rem 0.8rem;
    color: var(--muted);
    white-space: nowrap;
  }
  .tabs button.active {
    color: var(--text);
    border-bottom-color: var(--accent);
    font-weight: 600;
  }

  .log-head {
    display: flex;
    justify-content: space-between;
    padding: 0.35rem 0.75rem;
  }
  .log ol {
    list-style: none;
    margin: 0;
    padding: 0 0.75rem 0.6rem;
    overflow-y: auto;
    font-family: ui-monospace, monospace;
    font-size: 0.78rem;
  }
  .log li {
    padding: 0.15rem 0;
    overflow-wrap: anywhere;
  }
  .log time {
    color: var(--muted);
  }
  .log .ok {
    color: var(--ok);
  }
  .log .error {
    color: var(--error);
  }
  .log .muted {
    color: var(--muted);
  }
</style>
