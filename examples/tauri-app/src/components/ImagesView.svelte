<script>
  import { image, setLogo, logo, feed, cut, reset } from 'tauri-plugin-thermal-printer'
  import { SAMPLE_IMAGES, urlToDataUrl, fileToDataUrl, renderThermalPreview } from '../lib/images.js'
  import { app, pixelsWidth, printSections, log } from '../lib/state.svelte.js'

  let selectedId = $state(SAMPLE_IMAGES[0].id)
  let custom = $state(null) // { name, dataUrl }
  let maxWidth = $state(0)
  let align = $state('center')
  let dithering = $state(true)
  let size = $state('normal')
  let info = $state(null)
  let canvas = $state()

  const source = $derived(
    selectedId === 'custom' && custom ? custom.dataUrl : SAMPLE_IMAGES.find((i) => i.id === selectedId)?.url,
  )

  $effect(() => {
    if (!canvas || !source) return
    renderThermalPreview(canvas, source, { maxWidth: Number(maxWidth), dithering, paperWidth: pixelsWidth() })
      .then((r) => (info = r))
      .catch((e) => log('error', `Vista previa: ${e.message}`))
  })

  async function onFile(event) {
    const file = event.currentTarget.files?.[0]
    if (!file) return
    custom = { name: file.name, dataUrl: await fileToDataUrl(file) }
    selectedId = 'custom'
  }

  async function currentData() {
    return source.startsWith('data:') ? source : urlToDataUrl(source)
  }

  async function printImage() {
    const data = await currentData()
    printSections('Imagen', [
      reset(),
      image(data, { max_width: Number(maxWidth), align, dithering, size }),
      feed(3),
      cut(),
    ])
  }

  async function storeLogo() {
    const data = await currentData()
    printSections('Guardar logo NV', [setLogo(data, { max_width: Number(maxWidth), dithering })])
  }
</script>

<div class="layout">
  <div class="controls">
    <h2>Imagen</h2>
    <div class="gallery">
      {#each SAMPLE_IMAGES as img}
        <button class="thumb" class:active={selectedId === img.id} onclick={() => (selectedId = img.id)}>
          <img src={img.url} alt={img.name} />
          <span>{img.name}</span>
          <small>{img.hint}</small>
        </button>
      {/each}
      <label class="thumb upload" class:active={selectedId === 'custom'}>
        {#if custom}
          <img src={custom.dataUrl} alt={custom.name} />
          <span>{custom.name}</span>
        {:else}
          <span class="plus">+</span>
          <span>Subir imagen</span>
        {/if}
        <small>PNG, JPG, WEBP…</small>
        <input type="file" accept="image/*" onchange={onFile} />
      </label>
    </div>

    <h2>Opciones</h2>
    <div class="form">
      <label
        >Ancho máximo (px, 0 = papel)
        <input type="number" min="0" max={pixelsWidth()} step="8" bind:value={maxWidth} />
      </label>
      <label
        >Alineación
        <select bind:value={align}>
          <option value="left">Izquierda</option>
          <option value="center">Centro</option>
          <option value="right">Derecha</option>
        </select>
      </label>
      <label
        >Modo
        <select bind:value={size}>
          <option value="normal">Normal</option>
          <option value="double_width">Doble ancho</option>
          <option value="double_height">Doble alto</option>
          <option value="quadruple">Cuádruple</option>
        </select>
      </label>
      <label class="check"><input type="checkbox" bind:checked={dithering} /> Dithering (Floyd–Steinberg)</label>
    </div>

    <div class="actions">
      <button class="primary" disabled={app.busy || !source} onclick={printImage}>Imprimir imagen</button>
      <button disabled={app.busy || !source} onclick={storeLogo}>Guardar como logo NV</button>
      <button
        class="ghost"
        disabled={app.busy}
        onclick={() => printSections('Logo NV', [reset(), logo(1), feed(3), cut()])}>Imprimir logo NV</button
      >
    </div>
    <p class="note">
      El logo NV queda guardado en la memoria de la impresora (clave 1) y se imprime sin volver a enviar la imagen.
      No todas las genéricas lo soportan.
    </p>
  </div>

  <div class="preview">
    <h2>Vista previa térmica</h2>
    <div class="paper" style:width="{Math.min(pixelsWidth(), 576) / 1.5}px">
      <canvas bind:this={canvas} class:align-center={align === 'center'} class:align-right={align === 'right'}
      ></canvas>
    </div>
    {#if info}
      <p class="note">
        Original {info.original.width}×{info.original.height} px → impresa {info.width}×{info.height} px ·
        papel {pixelsWidth()} px
      </p>
    {/if}
  </div>
</div>

<style>
  .layout {
    display: grid;
    grid-template-columns: minmax(0, 1.3fr) minmax(0, 1fr);
    gap: 1.25rem;
  }
  @media (max-width: 760px) {
    .layout {
      grid-template-columns: 1fr;
    }
  }
  h2 {
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--muted);
    margin: 0 0 0.6rem;
  }
  .gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 0.6rem;
    margin-bottom: 1.2rem;
  }
  .thumb {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.6rem;
    border: 2px solid var(--border);
    background: var(--surface);
    border-radius: 12px;
    cursor: pointer;
    text-align: center;
    color: inherit;
    font: inherit;
  }
  .thumb.active {
    border-color: var(--accent);
  }
  .thumb img {
    width: 100%;
    height: 70px;
    object-fit: contain;
    background: #fff;
    border-radius: 6px;
  }
  .thumb span {
    font-size: 0.85rem;
    font-weight: 600;
    overflow-wrap: anywhere;
  }
  .thumb small {
    color: var(--muted);
    font-size: 0.72rem;
  }
  .upload input {
    display: none;
  }
  .plus {
    height: 70px;
    display: grid;
    place-items: center;
    font-size: 2rem !important;
    color: var(--muted);
  }
  .form {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  .form label {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    font-size: 0.85rem;
    color: var(--muted);
  }
  .form .check {
    flex-direction: row;
    align-items: center;
    color: var(--text);
  }
  .actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .note {
    color: var(--muted);
    font-size: 0.8rem;
  }
  .paper {
    max-width: 100%;
    background: #fff;
    padding: 12px 0;
    border-radius: 4px;
    box-shadow: 0 2px 12px #0003;
    display: flex;
    flex-direction: column;
  }
  canvas {
    max-width: 100%;
    image-rendering: pixelated;
    align-self: flex-start;
  }
  canvas.align-center {
    align-self: center;
  }
  canvas.align-right {
    align-self: flex-end;
  }
</style>
