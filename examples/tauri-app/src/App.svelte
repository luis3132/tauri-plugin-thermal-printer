<script>
  import {
    list_thermal_printers,
    print_thermal_printer,
    test_thermal_printer,
    title,
    subtitle,
    text,
    line,
    globalStyles,
    table,
    qr,
    barcode,
    beep2,
    reset,
    cut,
    setLogo,
  } from 'tauri-plugin-thermal-printer'

  // 1x1 PNG usado solo para demostrar el guardado de logo NV (FS q).
  const TINY_PNG =
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAADElEQVR4nGP4//8/AAX+Av4N70a4AAAAAElFTkSuQmCC'

  let printers = $state([])
  let selected = $state('')
  let paperSize = $state('Mm80')
  let log = $state('')

  function report(msg) {
    log += `[${new Date().toLocaleTimeString()}] ${msg}\n`
  }

  async function refresh() {
    try {
      printers = await list_thermal_printers()
      report(`Encontradas ${printers.length} impresora(s).`)
      if (!selected && printers.length > 0) selected = printers[0].identifier
    } catch (e) {
      report(`Error al listar: ${e}`)
    }
  }

  function baseRequest(sections) {
    return {
      printer: selected,
      sections,
      options: { code_page: 0 },
      paper_size: paperSize,
    }
  }

  // Recibo de demostración con tabla word-wrap, estilos, QR y código de barras.
  async function printReceipt() {
    if (!selected) return report('Selecciona una impresora primero.')
    const sections = [
      reset(),
      title('MI TIENDA'),
      subtitle('Recibo de compra'),
      line('='),
      table(
        3,
        [
          [text('Cafe Americano Grande Extra'), text('2'), text('$10.50')],
          [text('Pastel de chocolate'), text('1'), text('$25.00')],
        ],
        {
          header: [text('Producto'), text('Cant'), text('Precio')],
          column_widths: [24, 8, 16], // suma 48 (Mm80)
          truncate: false, // texto largo continúa DEBAJO
          word_wrap: true, // envuelve por palabra (opcional)
        },
      ),
      line('-'),
      globalStyles({ bold: true, align: 'right' }),
      text('TOTAL: $45.50'),
      globalStyles({ reset: true }),
      line('='),
      qr('https://github.com/luis3132/tauri-plugin-thermal-printer', { align: 'center' }),
      barcode('123456789012', 'CODE128', { align: 'center' }),
      text('¡Gracias por su compra!', { align: 'center' }),
      beep2(2, 3),
      cut('partial', 4),
    ]
    try {
      await print_thermal_printer(baseRequest(sections))
      report('Recibo enviado.')
    } catch (e) {
      report(`Error al imprimir: ${e}`)
    }
  }

  // Guarda un logo en memoria NV (FS q). Se imprime luego con logo().
  async function storeLogo() {
    if (!selected) return report('Selecciona una impresora primero.')
    try {
      await print_thermal_printer(baseRequest([setLogo(TINY_PNG)]))
      report('Logo guardado en memoria NV (clave 1).')
    } catch (e) {
      report(`Error al guardar logo: ${e}`)
    }
  }

  // Documento de prueba integral generado por el plugin.
  async function testPrint() {
    if (!selected) return report('Selecciona una impresora primero.')
    try {
      await test_thermal_printer({
        printer_info: baseRequest([]),
        test_logo: true,
        image_base64: TINY_PNG,
      })
      report('Prueba enviada.')
    } catch (e) {
      report(`Error en la prueba: ${e}`)
    }
  }
</script>

<main class="container">
  <h1>Thermal Printer — Demo</h1>

  <div class="row">
    <button onclick={refresh}>Buscar impresoras</button>
    <select bind:value={paperSize}>
      <option value="Mm58">58 mm</option>
      <option value="Mm80">80 mm</option>
    </select>
  </div>

  {#if printers.length > 0}
    <ul class="printers">
      {#each printers as p}
        <li>
          <label>
            <input type="radio" name="printer" value={p.identifier} bind:group={selected} />
            <strong>{p.name}</strong> — {p.interface_type} ({p.status})
          </label>
        </li>
      {/each}
    </ul>
  {:else}
    <p>No hay impresoras listadas. Pulsa «Buscar impresoras».</p>
  {/if}

  <div class="row">
    <button onclick={printReceipt} disabled={!selected}>Imprimir recibo</button>
    <button onclick={storeLogo} disabled={!selected}>Guardar logo (NV)</button>
    <button onclick={testPrint} disabled={!selected}>Prueba integral</button>
  </div>

  <pre class="log">{log}</pre>
</main>

<style>
  .container {
    max-width: 640px;
    margin: 0 auto;
    padding: 1rem;
    font-family: system-ui, sans-serif;
  }
  .row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin: 0.75rem 0;
    flex-wrap: wrap;
  }
  .printers {
    list-style: none;
    padding: 0;
    text-align: left;
  }
  .printers li {
    padding: 0.25rem 0;
  }
  .log {
    text-align: left;
    background: #1113;
    padding: 0.75rem;
    border-radius: 6px;
    min-height: 4rem;
    white-space: pre-wrap;
    font-size: 0.85rem;
  }
</style>
