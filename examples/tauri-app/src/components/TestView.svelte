<script>
  import { SAMPLE_IMAGES, urlToDataUrl } from '../lib/images.js'
  import { app, printTest } from '../lib/state.svelte.js'

  const FLAGS = [
    ['include_text', 'Texto básico'],
    ['include_text_styles', 'Estilos de texto'],
    ['include_alignment', 'Alineación'],
    ['include_columns', 'Columnas'],
    ['include_separators', 'Separadores'],
    ['include_barcode', 'Código de barras'],
    ['include_barcode_types', 'Todos los tipos de barras'],
    ['include_qr', 'QR'],
    ['include_image', 'Imagen'],
    ['test_all_fonts', 'Fuentes A/B/C'],
    ['test_invert', 'Invertido'],
    ['test_rotate', 'Rotado'],
    ['test_double_strike', 'Doble impresión'],
    ['test_spacing', 'Espaciado'],
    ['test_positioning', 'Tabulaciones y márgenes'],
    ['test_feed', 'Avance de papel'],
    ['include_beep', 'Pitido Epson'],
    ['test_beep2', 'Pitido genérico'],
    ['test_logo', 'Logo NV (guarda + imprime)'],
    ['test_cash_drawer', 'Abrir cajón'],
    ['cut_paper', 'Cortar papel'],
  ]

  const DEFAULT_ON = ['include_text', 'include_text_styles', 'include_alignment', 'include_columns',
    'include_separators', 'include_barcode', 'include_qr', 'include_image', 'cut_paper']

  let flags = $state(Object.fromEntries(FLAGS.map(([k]) => [k, DEFAULT_ON.includes(k)])))
  let customText = $state('')

  async function run() {
    const needsImage = flags.include_image || flags.test_logo
    const image_base64 = needsImage ? await urlToDataUrl(SAMPLE_IMAGES[0].url) : null
    printTest('Prueba integral', {
      ...flags,
      include_custom_text: customText.trim().length > 0,
      custom_text: customText.trim() || null,
      image_base64,
    })
  }

  function setAll(value) {
    for (const [k] of FLAGS) flags[k] = value
  }
</script>

<p class="intro">
  Usa <code>test_thermal_printer</code>: el plugin arma un documento de prueba con las secciones que marques. La
  imagen y el logo usan el logo de tienda de muestra.
</p>

<div class="toolbar">
  <button class="ghost" onclick={() => setAll(true)}>Marcar todo</button>
  <button class="ghost" onclick={() => setAll(false)}>Desmarcar todo</button>
</div>

<div class="flags">
  {#each FLAGS as [key, label]}
    <label><input type="checkbox" bind:checked={flags[key]} /> {label}</label>
  {/each}
</div>

<label class="custom">
  Texto personalizado (opcional)
  <textarea rows="3" bind:value={customText} placeholder="Se imprime en la prueba si no está vacío"></textarea>
</label>

<button class="primary big" disabled={app.busy} onclick={run}>Ejecutar prueba integral</button>

<style>
  .intro {
    color: var(--muted);
    margin-top: 0;
  }
  .toolbar {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }
  .flags {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
    gap: 0.4rem 1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 0.9rem;
  }
  .flags label {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.2rem 0;
  }
  .custom {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    margin: 1rem 0;
    color: var(--muted);
    font-size: 0.85rem;
  }
  .big {
    width: 100%;
    padding: 0.8rem;
    font-size: 1rem;
  }
</style>
